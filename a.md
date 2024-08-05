# Archive System

## Unresolved

- reconstruction
- redundancy
- cold vs hot
- wake on lan
- daemon-ization
- safe mode

## Host

### push

1. index files

2. preprocess files

3. complete archiving

    const CHUNK_MAX_SIZE=512MiB
    const MAX_MEMORY=2GiB
    const SAFE_MODE=false
    
    main thread (reads):
        1. read files into chunk buffers
        2. store hashes and indices locally
        3. delete current file when chunked
        4. hand buffers to zippers
    side threads (N) (zippers):
        1. compress files into new buffers
        2. hand buffers to net thread
    side thread (net):
        1. send data to client

#### IPC




## Store

var MOUNT_DIR=/archive

### structure:

{unique_hash}-{b64_dir}/
    meta.toml
    {b64_path}-{index}

### metadata:
    
- checksum
- dist partners
- original_size
- new_size

