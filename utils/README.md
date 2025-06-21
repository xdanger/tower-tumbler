# 开发工具和脚本

这个目录包含用于开发和调试的各种工具脚本。

## 📜 脚本列表

### 🌐 Web 服务器和构建

#### `start-trunk-server.sh`
启动 Trunk 开发服务器并记录日志到 `logs/server/`

```bash
cd utils && ./start-trunk-server.sh
```

- 自动创建日志目录
- 按日期保存服务器日志：`logs/server/trunk-YYYY-MM-DD.log`
- 同时在终端显示输出
- 无自动刷新模式，适合稳定开发

#### `build-release.sh`
构建生产版本并记录构建日志

```bash
cd utils && ./build-release.sh
```

- 自动创建日志目录
- 按时间戳保存构建日志：`logs/server/build-YYYY-MM-DD_HH-MM-SS.log`
- 显示构建结果和输出位置
- 构建失败时返回错误码

### 🔍 调试和监控

#### `log-server.py`
WebSocket 日志服务器，捕获浏览器控制台输出

```bash
cd utils && python3 log-server.py
```

**功能特性**：
- 监听端口：`8765`
- 日志保存到：`logs/browser/console-YYYY-MM-DD.log`
- 支持自动重连和离线缓存
- 实时捕获所有浏览器控制台输出

**捕获内容**：
- ✅ `console.log()` - 普通日志
- ✅ `console.info()` - 信息日志  
- ✅ `console.warn()` - 警告日志
- ✅ `console.error()` - 错误日志
- ✅ `console.debug()` - 调试日志
- ✅ **全局错误** - JavaScript 运行时错误
- ✅ **Promise 拒绝** - 未处理的 Promise 异常
- ✅ **资源加载错误** - 图片、脚本等加载失败

**优势**：
1. **实时监控**：不需要保持浏览器开发者工具打开
2. **持久保存**：所有日志都保存到文件，不会丢失
3. **远程调试**：可以监控移动设备上的浏览器
4. **自动重连**：网络断开后会自动重连
5. **缓冲机制**：离线时会缓存日志，连接恢复后发送

#### `start-log-server.sh`
启动浏览器日志服务器的便捷脚本

```bash
cd utils && ./start-log-server.sh
```

**安装依赖**：
```bash
pip install websockets
```

## 📂 日志文件组织

```
logs/
├── browser/          # 浏览器控制台日志
│   └── console-YYYY-MM-DD.log
└── server/           # 服务器和构建日志
    ├── trunk-YYYY-MM-DD.log
    └── build-YYYY-MM-DD_HH-MM-SS.log
```

## 🚀 典型开发流程

1. **启动日志监控**：
   ```bash
   cd utils && ./start-log-server.sh
   ```

2. **启动开发服务器**：
   ```bash
   cd utils && ./start-trunk-server.sh
   ```

3. **在浏览器中访问**：
   - http://localhost:8080

4. **查看实时日志**：
   - 服务器日志：`logs/server/trunk-$(date +%Y-%m-%d).log`
   - 浏览器日志：`logs/browser/console-$(date +%Y-%m-%d).log`

5. **构建生产版本**：
   ```bash
   cd utils && ./build-release.sh
   ```

## ⚙️ 配置说明

### 浏览器日志系统配置

可以修改 `index.html` 中的 `ConsoleLogger` 构造函数参数：

```javascript
const consoleLogger = new ConsoleLogger('ws://localhost:8765');  // 修改服务器地址
```

或者修改 `log-server.py` 中的设置：

```python
log_server = LogServer(host='localhost', port=8765, log_dir='../logs/browser')
```

**注意事项**：
- 日志服务器需要先启动，然后再打开浏览器
- 如果服务器没有运行，浏览器会每 5 秒尝试重连
- 日志缓冲区最多保存 100 条消息，防止内存溢出
- 所有原始的控制台功能都保持不变

### 环境变量
- `TRUNK_SERVE_PORT`：修改服务器端口（默认 8080）
- `TRUNK_BUILD_DIST`：修改构建输出目录（默认 dist）

### 自定义配置
所有脚本都可以通过修改脚本内的变量来自定义行为，或者通过环境变量覆盖默认设置。