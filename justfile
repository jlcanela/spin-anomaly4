build:
    @echo build

serve:
    cd frontend && trunk serve

up:
    spin up

watch:
    spin watch
    
# test everything
test-all:
    cargo test

# run a specific test
test TEST:
    cargo test {{ TEST }}
