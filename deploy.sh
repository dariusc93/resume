#!/bin/bash

BUCKET_NAME="<BUCKET_NAME>"
REGION="<REGION>"

echo "🦀 Building Yew portfolio..."
trunk build --release

echo "🪣 Deploying to S3..."
aws s3 sync dist/ s3://$BUCKET_NAME --delete

echo "🔧 Setting WASM MIME types..."
aws s3 cp dist/ s3://$BUCKET_NAME \
  --recursive \
  --exclude "*" \
  --include "*.wasm" \
  --content-type "application/wasm"

echo "✅ Deployment complete!"
echo "🌐 Website URL: http://$BUCKET_NAME.s3-website-$REGION.amazonaws.com"