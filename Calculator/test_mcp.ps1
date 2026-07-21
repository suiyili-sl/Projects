$process = New-Object System.Diagnostics.ProcessStartInfo
$process.FileName = "[path/to/Projects]\Calculator\target\debug\Calculator.exe"
$process.Arguments = "stdio"
$process.UseShellExecute = $false
$process.RedirectStandardInput = $true
$process.RedirectStandardOutput = $true

$proc = [System.Diagnostics.Process]::Start($process)

$stdin = $proc.StandardInput
$stdout = $proc.StandardOutput

# Send initialize request
$initialize = @{
    jsonrpc = "2.0"
    id = 1
    method = "initialize"
    params = @{
        protocolVersion = "2024-11-05"
        capabilities = @{}
        clientInfo = @{
            name = "test-client"
            version = "1.0.0"
        }
    }
} | ConvertTo-Json -Depth 10 -Compress

Write-Host "Sending initialize request..."
$stdin.WriteLine($initialize)
$stdin.Flush()

# Read response
$response = $stdout.ReadLine()
Write-Host "Response: $response"

# Send multiply tool call
$toolCall = @{
    jsonrpc = "2.0"
    id = 2
    method = "tools/call"
    params = @{
        name = "multiply"
        arguments = @{
            a = 52
            b = 39
        }
    }
} | ConvertTo-Json -Depth 10 -Compress

Write-Host "`nSending multiply(52, 39) request..."
$stdin.WriteLine($toolCall)
$stdin.Flush()

# Read response
$result = $stdout.ReadLine()
Write-Host "Result: $result"

# Cleanup
$stdin.Close()
$stdout.Close()
$proc.WaitForExit()

