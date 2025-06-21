// Script to fix WASM initialization and handle Bevy control flow exceptions
// This will be run after trunk build to patch the generated HTML

const fs = require('fs');
const path = require('path');

const distPath = path.join(__dirname, 'dist', 'index.html');

if (fs.existsSync(distPath)) {
    let html = fs.readFileSync(distPath, 'utf8');
    
    // Find the WASM initialization script
    const initPattern = /const wasm = await init\({ module_or_path: '\.\/[^']+' }\);/;
    
    if (html.match(initPattern)) {
        // Replace with exception-handling version
        html = html.replace(initPattern, `
try {
    const wasm = await init({ module_or_path: './tower-tumbler-c87fd431c2c3a71b_bg.wasm' });
} catch (error) {
    if (error.message && error.message.includes('Using exceptions for control flow')) {
        // This is expected Bevy control flow, continue execution
        console.log('Bevy application started successfully');
    } else {
        // Re-throw unexpected errors
        throw error;
    }
}`);
        
        fs.writeFileSync(distPath, html);
        console.log('Fixed WASM initialization to handle control flow exceptions');
    } else {
        console.log('Could not find WASM initialization pattern to fix');
    }
} else {
    console.log('dist/index.html not found');
}