#!/bin/bash

# Google Cloud Run Deployment Script for Rust WebTransport Server
# Usage: ./deploy-cloudrun.sh YOUR_PROJECT_ID

set -e

PROJECT_ID=$1
SERVICE_NAME="rust-wtransport-server"
REGION="us-central1"  # Change to your preferred region

if [ -z "$PROJECT_ID" ]; then
    echo "❌ Error: Please provide your Google Cloud Project ID"
    echo "Usage: ./deploy-cloudrun.sh YOUR_PROJECT_ID"
    exit 1
fi

echo "🚀 Deploying Rust WebTransport Server to Google Cloud Run"
echo "📋 Project ID: $PROJECT_ID"
echo "🌍 Region: $REGION"
echo "🔧 Service Name: $SERVICE_NAME"
echo ""

# Set the project
echo "🔧 Setting up Google Cloud project..."
gcloud config set project $PROJECT_ID

# Enable required APIs
echo "🔌 Enabling required APIs..."
gcloud services enable cloudbuild.googleapis.com
gcloud services enable run.googleapis.com
gcloud services enable containerregistry.googleapis.com

# Build the container
echo "🏗️ Building container image..."
gcloud builds submit --tag gcr.io/$PROJECT_ID/$SERVICE_NAME

# Deploy to Cloud Run
echo "🚀 Deploying to Cloud Run..."
gcloud run deploy $SERVICE_NAME \
    --image gcr.io/$PROJECT_ID/$SERVICE_NAME \
    --platform managed \
    --region $REGION \
    --allow-unauthenticated \
    --port 8080 \
    --memory 512Mi \
    --cpu 1 \
    --concurrency 1000 \
    --timeout 3600 \
    --min-instances 1 \
    --max-instances 10

# Get the service URL
SERVICE_URL=$(gcloud run services describe $SERVICE_NAME --platform managed --region $REGION --format 'value(status.url)')

echo ""
echo "✅ Deployment successful!"
echo "🌐 Your WebTransport server is live at:"
echo "   $SERVICE_URL"
echo ""
echo "🔧 Next steps:"
echo "1. Update your frontend to use this URL: $SERVICE_URL"
echo "2. Test WebTransport connection"
echo "3. 🎮 Start gaming!"
echo "" 