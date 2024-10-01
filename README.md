## The client and host was implemented as a single project
- Follow instructions below to select which one to run
- I did not implement a stay alive connection or automatic hardware data updating, to get latest hardware info you must run client again

1. Install postgres docker image (instructions did not indicate any datastore so using postgres with docker)
    1. open command line (not powershell) 
    2. cd to service
    3. run cargo install sqlx-cli
    4. run set DATABASE_URL=postgres://vr360action:vr360action@localhost/vr360action
    5. run dockerbuild.cmd
    6. sqlx migrate run --database-url postgres://vr360action:vr360action@localhost:5432/vr360action
1. Run host
    1. open command line (not powershell)
    2. cd to target/debug
    3. run set HOST=127.0.0.1 && set PORT=5001 && service
2. Run client
    1. open command line (not powershell)
    2. cd to target/debug
    3. run set HOST=127.0.0.1 && set PORT=5001 && set CLIENT=true && service
