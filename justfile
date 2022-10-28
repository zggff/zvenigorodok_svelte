build:
    #!/usr/bin/env zsh
    cd frontend
    yarn 
    yarn build

    cd ../backend
    cross build --target x86_64-unknown-linux-gnu --release

    cd ..
    rm -rf release
    mkdir release
    cp -r frontend/build release/public
    cp backend/target/x86_64-unknown-linux-gnu/release/backend release/backend

deploy: build
   scp -r ./release root@79.143.31.195:/root/zvenigorodok/

