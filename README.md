# Banner Container

## Overview
This directory contains the Containerfile and source code necessary to create the single route Closed Circuit Consulting banner HTTP server container image.

Using this image is helpful to:
* Prove that container networking is working on the host
* The containers published port is reachable from the host (`curl http://localhost:8080`)
* The containers published port is reachable from machines other than the host (if allowed) (`curl http://<ip_address>:8080`)
* The container runs as a service that can be started, stopped and restarted (`podman stop` shuts it down gracefully)
* Systemd and Quadlet readiness notification is working (`Notify=true`)
* Provide a simple backend for testing reverse proxies, load balancers and firewall rules

## Run
Run:
```bash
podman run -d --name banner -p 8080:8080 ghcr.io/closedcircuitconsulting/banner-rs
```

Get the banner:
```bash
curl http://localhost:8080
```

Which will output:
```

          ######   ######   ######  
         ##    ## ##    ## ##    ## 
         ##       ##       ##       
         ##       ##       ##       
         ##       ##       ##       
         ##    ## ##    ## ##    ## 
          ######   ######   ######  

          Closed Circuit Consulting
               Twin Cities, MN            
  
  https://git.closedcircuitconsulting.com
  https://github.com/closedcircuitconsulting

```

Stop and remove it:
```bash
podman rm -f banner
```

## Build
Build:
```bash
podman build -t cccbanner .
```

Run:
```bash
podman run -d --name banner -p 8080:8080 localhost/cccbanner:latest
```
