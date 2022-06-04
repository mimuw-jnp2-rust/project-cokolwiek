#!/bin/sh

# https://coqui.ai/polish/jaco-assistant/v0.0.1

echo "Downloading the alphabet..."
wget https://coqui.gateway.scarf.sh/polish/jaco-assistant/v0.0.1/alphabet.txt
echo "Downloading the model itself..."
wget https://coqui.gateway.scarf.sh/polish/jaco-assistant/v0.0.1/model.tflite
echo "Downloading the scorer (this may take a while)"
wget https://coqui.gateway.scarf.sh/polish/jaco-assistant/v0.0.1/kenlm_pl.scorer
