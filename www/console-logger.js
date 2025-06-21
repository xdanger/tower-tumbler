// Browser Console Logger - Captures all console output and sends to WebSocket
class ConsoleLogger {
    constructor(wsUrl = 'ws://localhost:8765') {
        this.wsUrl = wsUrl;
        this.ws = null;
        this.buffer = [];
        this.connected = false;
        this.reconnectInterval = 5000;
        
        this.init();
    }
    
    init() {
        this.setupConsoleCapture();
        this.setupErrorCapture();
        this.connect();
    }
    
    connect() {
        try {
            this.ws = new WebSocket(this.wsUrl);
            
            this.ws.onopen = () => {
                this.connected = true;
                console.log('[ConsoleLogger] Connected to log server');
                this.flushBuffer();
            };
            
            this.ws.onclose = () => {
                this.connected = false;
                console.log('[ConsoleLogger] Disconnected from log server, will retry...');
                setTimeout(() => this.connect(), this.reconnectInterval);
            };
            
            this.ws.onerror = (error) => {
                console.log('[ConsoleLogger] WebSocket error:', error);
            };
        } catch (error) {
            console.log('[ConsoleLogger] Failed to connect:', error);
            setTimeout(() => this.connect(), this.reconnectInterval);
        }
    }
    
    log(level, args) {
        const timestamp = new Date().toISOString();
        const message = {
            timestamp,
            level,
            args: Array.from(args).map(arg => {
                if (typeof arg === 'object') {
                    try {
                        return JSON.stringify(arg, null, 2);
                    } catch (e) {
                        return arg.toString();
                    }
                }
                return String(arg);
            }),
            url: window.location.href,
            userAgent: navigator.userAgent
        };
        
        if (this.connected && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify(message));
        } else {
            this.buffer.push(message);
            // Keep buffer size manageable
            if (this.buffer.length > 100) {
                this.buffer.shift();
            }
        }
    }
    
    flushBuffer() {
        while (this.buffer.length > 0 && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify(this.buffer.shift()));
        }
    }
    
    setupConsoleCapture() {
        const originalMethods = {};
        const levels = ['log', 'info', 'warn', 'error', 'debug'];
        
        levels.forEach(level => {
            originalMethods[level] = console[level];
            console[level] = (...args) => {
                // Call original method first
                originalMethods[level].apply(console, args);
                // Then log to our system
                this.log(level, args);
            };
        });
    }
    
    setupErrorCapture() {
        // Global error handler
        window.addEventListener('error', (event) => {
            this.log('error', [
                'Global Error:',
                event.error ? event.error.stack || event.error.message : event.message,
                'at', event.filename + ':' + event.lineno + ':' + event.colno
            ]);
        });
        
        // Unhandled promise rejection handler
        window.addEventListener('unhandledrejection', (event) => {
            this.log('error', [
                'Unhandled Promise Rejection:',
                event.reason && event.reason.stack ? event.reason.stack : event.reason
            ]);
        });
        
        // Resource loading errors
        window.addEventListener('error', (event) => {
            if (event.target !== window) {
                this.log('error', [
                    'Resource Load Error:',
                    event.target.tagName,
                    event.target.src || event.target.href || 'unknown source'
                ]);
            }
        }, true);
    }
}

// Auto-initialize if this script is loaded
if (typeof window !== 'undefined') {
    window.consoleLogger = new ConsoleLogger();
}