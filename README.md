# vpn-port-forward-manager

## Environment Variables

| Variable Name     | Default Value               | Description                                                   |
|-------------------|-----------------------------|---------------------------------------------------------------|
| QB_PROTOCOL       | http                        | Web UI protcol (`http` or `https`)                            |
| QB_HOST           | localhost                   | Web UI hostname                                               |
| QB_PORT           | 80                          | Web UI port                                                   |
| QB_USER           | admin                       | Web UI user name                                              |
| QB_PASSWORD       | blank                       | Web UI user password                                          |
| PORT_FORWARD_PATH | /tmp/gluetun/forwarded_port | Path to the file containing the port value                    |
| CHECK_INTERVAL    | 20                          | Time between checks in seconds                                |
| LOG_LEVEL         | info                        | Set logging level (`error`, `warn`, `info`, `debug`, `trace`) |
