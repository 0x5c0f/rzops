#!/bin/sh
echo '=== providers script tags ==='
curl -s 'http://localhost:5173/providers' | grep -o '<script[^>]*src="[^"]*"' | head -5
echo '=== providers/new script tags ==='
curl -s 'http://localhost:5173/providers/new' | grep -o '<script[^>]*src="[^"]*"' | head -5
echo '=== providers/new full scripts ==='
curl -s 'http://localhost:5173/providers/new' | grep -o '<script[^>]*>' | head -10
echo '=== tail of providers/new html ==='
curl -s 'http://localhost:5173/providers/new' | tail -c 600
