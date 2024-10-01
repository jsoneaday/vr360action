## The client and host was implemented as a single project
- Follow instructions below to select which one to run
- I did not implement a stay alive connection or automatic hardware data updating, to get latest hardware info you must run client again

1. Install postgres (instructions did not indicate any datastore so using postgres with docker)
    1. open command line (not powershell) 
    2. cd to target/debug
    3. run set DATABASE_URL=postgres://vr360action:vr360action@localhost/vr360action
1. Run host
    1. open command line (not powershell)
    2. cd to target/debug
    3. run set HOST=127.0.0.1 && set PORT=5001 && service
2. Run client
    1. open command line (not powershell)
    2. cd to target/debug
    3. run set HOST=127.0.0.1 && set PORT=5001 && set CLIENT=true && service
