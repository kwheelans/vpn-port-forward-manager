# vpn-port-forward-manager

## Environment Variables

| Variable Name     | Description                                | Values                                    |
|-------------------|--------------------------------------------|-------------------------------------------|
| APPLICATION       | The application to update the port for     | `qBittorent`, `Deluge`                    |
| PROTOCOL          | Protocal used to access the host           | `http`, `https`                           |
| HOST              | Hostname ie. `app.example.com`             | String                                    |
| PORT              | Port used to acces the host                | Unsigned Integer                          |
| USER              | User name to access the host application   | String                                    |
| PASSWORD          | Password to access the host application    | String                                    |
| PORT_FORWARD_PATH | Path to the file containing the port value | String                                    |
| CHECK_INTERVAL    | Time between checks in seconds             | Unsigned Integer                          |
| LOG_LEVEL         | Set logging level                          | `error`, `warn`, `info`, `debug`, `trace` |

### Common Default Values
| Variable Name     | Default Value               |
|-------------------|-----------------------------|
| PROTOCOL          | `http`                      |
| HOST              | `localhost`                 |
| PORT_FORWARD_PATH | /tmp/gluetun/forwarded_port |
| CHECK_INTERVAL    | 20                          |
| LOG_LEVEL         | info                        |

### qBittorrent Default Values
| Variable Name | Default Value |
|---------------|---------------|
| PORT          | `8080`        |
| USER          | `admin`       |
| PASSWORD      | blank         |

### Deluge Default Values
| Variable Name | Default Value |
|---------------|---------------|
| PORT          | `8080`        |
| USER          | `admin`       |
| PASSWORD      | blank         |
