package main

import (
	"bytes"
	"fmt"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/spf13/cobra"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestCLIHelp(t *testing.T) {
	// Test the actual rootCmd help
	var buf bytes.Buffer
	rootCmd.SetOut(&buf)
	rootCmd.SetArgs([]string{"--help"})

	err := rootCmd.Execute()
	require.NoError(t, err)

	output := buf.String()
	assert.Contains(t, output, "ApeCloud Data Transfer Service")
	assert.Contains(t, output, "snapshot")
	assert.Contains(t, output, "cdc")
	assert.Contains(t, output, "check")
	assert.Contains(t, output, "struct")
}

func TestCLIVersion(t *testing.T) {
	// Reset rootCmd to avoid conflicts
	originalCmd := rootCmd
	defer func() { rootCmd = originalCmd }()

	// Create a new command for testing
	testCmd := &cobra.Command{
		Use:     "ape-dts",
		Version: version,
	}

	var buf bytes.Buffer
	testCmd.SetOut(&buf)
	testCmd.SetArgs([]string{"--version"})

	err := testCmd.Execute()
	require.NoError(t, err)

	output := buf.String()
	assert.Contains(t, output, "2.0.0-go")
}

func TestValidateConfig(t *testing.T) {
	tests := []struct {
		name        string
		configFile  string
		expectError bool
		setupFunc   func(string) error
	}{
		{
			name:        "valid config",
			configFile:  "valid_config.ini",
			expectError: false,
			setupFunc: func(file string) error {
				content := `[extractor]
db_type=mysql
extract_type=snapshot

[sinker]
db_type=mysql
sink_type=write

[runtime]
[parallelizer]
[pipeline]
[filter]
[router]
[resumer]`
				return os.WriteFile(file, []byte(content), 0644)
			},
		},
		{
			name:        "nonexistent config",
			configFile:  "nonexistent.ini",
			expectError: true,
			setupFunc:   func(string) error { return nil },
		},
		{
			name:        "invalid config",
			configFile:  "invalid_config.ini",
			expectError: true,
			setupFunc: func(file string) error {
				content := `[extractor]
db_type=invalid_db_type`
				return os.WriteFile(file, []byte(content), 0644)
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			tmpDir := t.TempDir()
			configPath := filepath.Join(tmpDir, tt.configFile)

			err := tt.setupFunc(configPath)
			require.NoError(t, err)

			if tt.name == "nonexistent config" {
				// Don't create the file for this test
				err = validateConfig(configPath, false)
			} else {
				err = validateConfig(configPath, false)
			}

			if tt.expectError {
				assert.Error(t, err)
			} else {
				assert.NoError(t, err)
			}
		})
	}
}

func TestValidateConfigDetailed(t *testing.T) {
	tmpDir := t.TempDir()
	configFile := filepath.Join(tmpDir, "test_config.ini")

	configContent := `[extractor]
db_type=postgres
extract_type=cdc
url=postgresql://user:pass@localhost:5432/sourcedb

[sinker]
db_type=mysql
sink_type=write
url=mysql://user:pass@localhost:3306/targetdb
batch_size=2000

[runtime]
log_level=debug

[parallelizer]
parallel_type=snapshot
parallel_size=8

[pipeline]
pipeline_type=http_server
buffer_size=32000

[filter]
[router]
[resumer]`

	err := os.WriteFile(configFile, []byte(configContent), 0644)
	require.NoError(t, err)

	// Capture output for detailed validation
	originalStdout := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w

	err = validateConfig(configFile, true)
	w.Close()

	output := make([]byte, 1000)
	n, _ := r.Read(output)
	os.Stdout = originalStdout

	require.NoError(t, err)
	outputStr := string(output[:n])
	assert.Contains(t, outputStr, "Extract type: cdc")
	assert.Contains(t, outputStr, "Source DB type: pg")
	assert.Contains(t, outputStr, "Target DB type: mysql")
}

func TestExecuteLegacyMode(t *testing.T) {
	tmpDir := t.TempDir()
	configFile := filepath.Join(tmpDir, "legacy_config.ini")

	configContent := `[extractor]
db_type=mysql
extract_type=snapshot

[sinker]
db_type=mysql
sink_type=write
batch_size=1000

[runtime]
[parallelizer]
[pipeline]
[filter]
[router]
[resumer]`

	err := os.WriteFile(configFile, []byte(configContent), 0644)
	require.NoError(t, err)

	// Test normal execution
	err = executeLegacyMode(configFile)
	assert.NoError(t, err)

	// Test dry-run mode
	dryRun = true
	defer func() { dryRun = false }()

	err = executeLegacyMode(configFile)
	assert.NoError(t, err)
}

func TestExecuteTaskOperation(t *testing.T) {
	tmpDir := t.TempDir()
	configFile := filepath.Join(tmpDir, "task_config.ini")

	configContent := `[extractor]
db_type=mysql
extract_type=snapshot
url=mysql://localhost:3306/test

[sinker]
db_type=mysql
sink_type=write
url=mysql://localhost:3307/target
batch_size=500

[runtime]
[parallelizer]
parallel_size=2
[pipeline]
[filter]
[router]
[resumer]`

	err := os.WriteFile(configFile, []byte(configContent), 0644)
	require.NoError(t, err)

	err = executeTaskOperation(configFile)
	assert.NoError(t, err)
}

func TestDisplayInfo(t *testing.T) {
	tmpDir := t.TempDir()
	configFile := filepath.Join(tmpDir, "info_config.ini")

	configContent := `[extractor]
db_type=postgres
extract_type=cdc
url=postgresql://user:pass@localhost:5432/source

[sinker]
db_type=clickhouse
sink_type=write
url=http://localhost:8123
batch_size=10000

[runtime]
[parallelizer]
parallel_size=16
[pipeline]
buffer_size=50000
buffer_memory_mb=1024
max_rps=5000
checkpoint_interval_secs=30
[filter]
[router]
[resumer]`

	err := os.WriteFile(configFile, []byte(configContent), 0644)
	require.NoError(t, err)

	// Capture output
	originalStdout := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w

	err = displayInfo(configFile, true, true)
	w.Close()

	output := make([]byte, 2000)
	n, _ := r.Read(output)
	os.Stdout = originalStdout

	require.NoError(t, err)
	outputStr := string(output[:n])

	assert.Contains(t, outputStr, "Configuration Information")
	assert.Contains(t, outputStr, "Extract type: cdc")
	assert.Contains(t, outputStr, "Source DB: pg")
	assert.Contains(t, outputStr, "Target DB: clickhouse")
	assert.Contains(t, outputStr, "Connection Information")
	assert.Contains(t, outputStr, "Performance Settings")
	assert.Contains(t, outputStr, "Buffer size: 50000")
	assert.Contains(t, outputStr, "Buffer memory: 1024MB")
}

func TestMaskPassword(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "empty url",
			input:    "",
			expected: "",
		},
		{
			name:     "normal url",
			input:    "mysql://user:pass@localhost:3306/db",
			expected: "mysql://user:pass@localhost:3306/db", // For now, returns as-is
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := maskPassword(tt.input)
			assert.Equal(t, tt.expected, result)
		})
	}
}

func TestSubcommandExecution(t *testing.T) {
	tmpDir := t.TempDir()
	configFile := filepath.Join(tmpDir, "subcommand_config.ini")

	configContent := `[extractor]
db_type=mysql
extract_type=snapshot

[sinker]
db_type=mysql
sink_type=write

[runtime]
[parallelizer]
[pipeline]
[filter]
[router]
[resumer]`

	err := os.WriteFile(configFile, []byte(configContent), 0644)
	require.NoError(t, err)

	// Test various subcommands
	subcommands := []string{"snapshot", "cdc", "check", "struct", "review", "revise"}

	for _, subcmd := range subcommands {
		t.Run(fmt.Sprintf("subcommand_%s", subcmd), func(t *testing.T) {
			// Set up test environment
			originalConfig := configFile
			configFile = configFile
			defer func() { configFile = originalConfig }()

			// Test dry-run mode for each subcommand
			originalDryRun := dryRun
			dryRun = true
			defer func() { dryRun = originalDryRun }()

			var err error
			switch subcmd {
			case "snapshot":
				err = snapshotCmd.RunE(snapshotCmd, []string{})
			case "cdc":
				err = cdcCmd.RunE(cdcCmd, []string{})
			case "check":
				err = checkCmd.RunE(checkCmd, []string{})
			case "struct":
				err = structCmd.RunE(structCmd, []string{})
			case "review":
				err = reviewCmd.RunE(reviewCmd, []string{})
			case "revise":
				err = reviseCmd.RunE(reviseCmd, []string{})
			}

			assert.NoError(t, err, "subcommand %s should not error in dry-run mode", subcmd)
		})
	}
}

func TestGetShutdownTimeout(t *testing.T) {
	// Test default timeout
	originalEnv := os.Getenv(envShutdownTimeoutSecs)
	defer os.Setenv(envShutdownTimeoutSecs, originalEnv)

	os.Unsetenv(envShutdownTimeoutSecs)
	timeout := getShutdownTimeout()
	assert.Equal(t, 3*time.Second, timeout)

	// Test custom timeout
	os.Setenv(envShutdownTimeoutSecs, "10")
	timeout = getShutdownTimeout()
	assert.Equal(t, 10*time.Second, timeout)

	// Test invalid timeout (should default)
	os.Setenv(envShutdownTimeoutSecs, "invalid")
	timeout = getShutdownTimeout()
	assert.Equal(t, 3*time.Second, timeout)
}

func TestFlagValidation(t *testing.T) {
	// Test that all subcommands have the expected flags
	tests := []struct {
		cmd           *cobra.Command
		expectedFlags []string
	}{
		{
			cmd:           snapshotCmd,
			expectedFlags: []string{"source", "target", "batch-size", "parallel"},
		},
		{
			cmd:           cdcCmd,
			expectedFlags: []string{"source", "target", "position", "heartbeat-table"},
		},
		{
			cmd:           checkCmd,
			expectedFlags: []string{"source", "target", "mode", "output"},
		},
		{
			cmd:           structCmd,
			expectedFlags: []string{"source", "target", "include-data", "drop-if-exists"},
		},
		{
			cmd:           reviewCmd,
			expectedFlags: []string{"source", "target", "depth", "html-report"},
		},
		{
			cmd:           reviseCmd,
			expectedFlags: []string{"source", "target", "strategy", "backup"},
		},
		{
			cmd:           validateCmd,
			expectedFlags: []string{"detailed"},
		},
		{
			cmd:           infoCmd,
			expectedFlags: []string{"connections", "metrics"},
		},
	}

	for _, tt := range tests {
		t.Run(fmt.Sprintf("flags_%s", tt.cmd.Name()), func(t *testing.T) {
			for _, flagName := range tt.expectedFlags {
				flag := tt.cmd.Flags().Lookup(flagName)
				assert.NotNil(t, flag, "flag %s should exist on command %s", flagName, tt.cmd.Name())
			}
		})
	}
}

func TestGlobalFlags(t *testing.T) {
	globalFlags := []string{"config", "log-level", "dry-run", "verbose"}

	for _, flagName := range globalFlags {
		t.Run(fmt.Sprintf("global_flag_%s", flagName), func(t *testing.T) {
			flag := rootCmd.PersistentFlags().Lookup(flagName)
			assert.NotNil(t, flag, "global flag %s should exist", flagName)
		})
	}
}

func TestCommandDescriptions(t *testing.T) {
	commands := []*cobra.Command{
		snapshotCmd, cdcCmd, checkCmd, structCmd,
		reviewCmd, reviseCmd, validateCmd, infoCmd,
	}

	for _, cmd := range commands {
		t.Run(fmt.Sprintf("description_%s", cmd.Name()), func(t *testing.T) {
			assert.NotEmpty(t, cmd.Short, "command %s should have a short description", cmd.Name())
			assert.NotEmpty(t, cmd.Long, "command %s should have a long description", cmd.Name())
			assert.True(t, len(cmd.Long) > len(cmd.Short), "long description should be longer than short for %s", cmd.Name())
		})
	}
}