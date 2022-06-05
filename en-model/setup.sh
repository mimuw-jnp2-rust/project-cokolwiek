#!/bin/sh

# https://coqui.ai/english/coqui/v0.9.3

echo "Downloading the alphabet..."
wget https://coqui.gateway.scarf.sh/english/coqui/v0.9.3/alphabet.txt
# todo (or rather to-research): .tflite, perhaps .pbmm would work better?
echo "Downloading the model itself..."
wget https://coqui.gateway.scarf.sh/english/coqui/v0.9.3/model.tflite
echo "Downloading the scorer (this may take a while (I mean a WHILE))"
wget https://coqui.gateway.scarf.sh/english/coqui/v0.9.3/coqui-stt-0.9.3-models.scorer
