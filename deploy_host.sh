#!/bin/bash
# created by Davide Rovelli
# daft script to deploy source when testing from local machines.
# assumes SSH key auth & DEST directory existing

EXCLUDE_LIST="{'test/','deploy.sh','example.asgard-bench.ini','.vscode','.config.hash'}"
DEST="/mnt/data/drovelli9/dmem-test"
USER=""
NODES=("cxlnode")

# for each node, execute rsync
for node in ${NODES[@]}
do
    echo "Deploying to $node"
    rsync -r ./ $node:$DEST --exclude $EXCLUDE_LIST
done