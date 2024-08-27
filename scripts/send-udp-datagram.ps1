Param (
    [string] $EndPoint, 
    [int] $Port, 
    [string] $Message
)

$client = new-object net.sockets.udpclient($Port)

write-host "You are $(((ipconfig) -match 'IPv').split(':')[1].trim()):$($client.client.localendpoint.port)"

$peerIP = read-host "Peer IP address"
$peerPort = read-host "Peer port"

$send = [text.encoding]::ascii.getbytes($Message)
[void] $client.send($send, $send.length, $peerIP, $peerPort)

$ipep = new-object net.ipendpoint([net.ipaddress]::any, 0)
$receive = $client.receive([ref]$ipep)

echo ([text.encoding]::ascii.getstring($receive))

$client.close()