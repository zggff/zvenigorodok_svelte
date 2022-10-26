deploy_frontend:
    #!/usr/bin/env zsh
    cd frontend 
    yarn
    yarn build
    yarn deploy
