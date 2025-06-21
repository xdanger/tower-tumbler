#!/usr/bin/env python3
"""
WebSocket Log Server for Browser Console Logs
Receives logs from browser and writes them to files
"""

import asyncio
import websockets
import json
import os
import datetime
from pathlib import Path

class LogServer:
    def __init__(self, host='localhost', port=8765, log_dir='../logs/browser'):
        self.host = host
        self.port = port
        self.log_dir = Path(log_dir)
        self.log_dir.mkdir(exist_ok=True)
        self.connections = set()
        
    async def register(self, websocket):
        self.connections.add(websocket)
        print(f"Client connected: {websocket.remote_address}")
        
    async def unregister(self, websocket):
        self.connections.discard(websocket)
        print(f"Client disconnected: {websocket.remote_address}")
        
    def write_log(self, log_data):
        try:
            timestamp = datetime.datetime.now()
            date_str = timestamp.strftime('%Y-%m-%d')
            time_str = timestamp.strftime('%H:%M:%S.%f')[:-3]
            
            # Create daily log file
            log_file = self.log_dir / f"console-{date_str}.log"
            
            # Format log entry
            level = log_data.get('level', 'unknown').upper()
            args = log_data.get('args', [])
            message = ' '.join(str(arg) for arg in args)
            url = log_data.get('url', 'unknown')
            
            log_entry = f"[{time_str}] [{level}] {message} (from: {url})\n"
            
            # Write to file
            with open(log_file, 'a', encoding='utf-8') as f:
                f.write(log_entry)
                
            # Also print to console
            print(f"[{time_str}] [{level}] {message}")
            
        except Exception as e:
            print(f"Error writing log: {e}")
    
    async def handle_client(self, websocket):
        await self.register(websocket)
        try:
            async for message in websocket:
                try:
                    log_data = json.loads(message)
                    self.write_log(log_data)
                except json.JSONDecodeError as e:
                    print(f"Invalid JSON received: {e}")
                except Exception as e:
                    print(f"Error processing message: {e}")
        except websockets.exceptions.ConnectionClosed:
            pass
        except Exception as e:
            print(f"Error in handle_client: {e}")
        finally:
            await self.unregister(websocket)
    
    async def start_server(self):
        print(f"Starting log server on {self.host}:{self.port}")
        print(f"Logs will be written to: {self.log_dir.absolute()}")
        
        # Use the newer websockets.serve syntax
        async with websockets.serve(
            self.handle_client, 
            self.host, 
            self.port,
            ping_interval=30,
            ping_timeout=10
        ) as server:
            await asyncio.Future()  # Run forever

async def main():
    log_server = LogServer()
    
    print("Log server is running. Press Ctrl+C to stop.")
    
    try:
        await log_server.start_server()
    except KeyboardInterrupt:
        print("\nShutting down log server...")

if __name__ == "__main__":
    asyncio.run(main())