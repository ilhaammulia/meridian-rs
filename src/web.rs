use axum::{
    routing::get,
    Router,
    response::Html,
    Json,
};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};

pub async fn start_web_server() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(main_page))
        .route("/api/status", get(status))
        .route("/api/positions", get(get_positions))
        .route("/api/balance", get(get_balance))
        .route("/api/screening", get(get_screening))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("[Meridian OS] Running on http://localhost:3000");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn main_page() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Meridian OS</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <style>
        body { font-family: 'JetBrains Mono', 'SF Mono', monospace; background: #0a0a0a; }
        .terminal { background: #111113; border: 1px solid #27272a; }
        .section { font-size: 12px; letter-spacing: 0.5px; color: #64748b; }
    </style>
</head>
<body class="bg-zinc-950 text-zinc-200 p-8">
    <div class="max-w-screen-2xl mx-auto">
        <div class="mb-8">
            <div class="text-5xl font-semibold tracking-tight">Meridian OS</div>
            <div class="text-zinc-400 mt-1">DLMM Liquidity Agent</div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            
            <!-- Terminal -->
            <div>
                <div class="section mb-3 flex items-center gap-x-2">
                    <i class="fa-solid fa-terminal"></i>
                    <span>TERMINAL</span>
                </div>
                <div class="terminal rounded-3xl p-6 h-[580px] flex flex-col">
                    <div class="flex-1 overflow-auto text-sm" id="term-output">
                        <div class="text-emerald-400">Meridian Terminal v0.2.0</div>
                        <div class="text-zinc-500">Type 'help'</div><br>
                    </div>
                    <div class="flex items-center border-t border-zinc-800 pt-4">
                        <span class="text-emerald-400 mr-3">$</span>
                        <input id="term-input" class="flex-1 bg-transparent outline-none text-white text-sm" 
                               placeholder="Type command..." onkeydown="if(event.key==='Enter') runTerm()">
                    </div>
                </div>
            </div>

            <!-- Cycle Log -->
            <div>
                <div class="section mb-3 flex items-center gap-x-2">
                    <i class="fa-solid fa-sync"></i>
                    <span>CYCLE LOG</span>
                </div>
                <div class="terminal rounded-3xl p-6 h-[580px] flex flex-col">
                    <div class="flex-1 overflow-auto text-sm" id="cycle-output">
                        <div class="text-emerald-400">Cycle Logger v0.1.0</div>
                        <div class="text-zinc-500">Waiting for cycles...</div><br>
                    </div>
                    <div class="pt-4 border-t border-zinc-800">
                        <button onclick="startCycles()" 
                                class="w-full py-3 bg-emerald-600 hover:bg-emerald-700 rounded-2xl text-sm font-semibold">
                            Start Cycles
                        </button>
                    </div>
                </div>
            </div>

        </div>
    </div>

    <script>
        // Terminal
        const termOut = document.getElementById('term-output');
        const termIn = document.getElementById('term-input');

        function printTerm(text, color = '') {
            const line = document.createElement('div');
            line.className = color;
            line.innerHTML = text;
            termOut.appendChild(line);
            termOut.scrollTop = termOut.scrollHeight;
        }

        function runTerm() {
            const cmd = termIn.value.trim();
            if (!cmd) return;
            printTerm(`$ ${cmd}`, 'text-emerald-400');

            if (cmd === 'help') {
                printTerm('status | positions | screening | balance | clear | help');
            } else if (cmd === 'status') {
                printTerm('Agent: RUNNING | Positions: 3 | Last: 2m ago');
            } else if (cmd === 'positions') {
                printTerm('MEME 1.24 SOL | CAT 0.87 SOL');
            } else if (cmd === 'clear') {
                termOut.innerHTML = '';
            } else {
                printTerm('Unknown command');
            }
            termIn.value = '';
        }

        // Cycle Log
        const cycleOut = document.getElementById('cycle-output');

        function printCycle(text, color = '') {
            const line = document.createElement('div');
            line.className = color;
            line.innerHTML = text;
            cycleOut.appendChild(line);
            cycleOut.scrollTop = cycleOut.scrollHeight;
        }

        function startCycles() {
            printCycle('[Cycle] Starting scheduler...', 'text-emerald-400');
            setTimeout(() => printCycle('[Management] Running cycle...'), 800);
            setTimeout(() => printCycle('[Screening] Running cycle...'), 1600);
            setTimeout(() => printCycle('[Cycle] Completed'), 2400);
        }

        setTimeout(() => {
            printTerm('Welcome to Meridian Terminal', 'text-emerald-400');
            printCycle('Cycle logger ready', 'text-emerald-400');
        }, 500);

        window.onload = () => termIn.focus();
    </script>
</body>
</html>
    "#)
}

// API
async fn status() -> Json<serde_json::Value> {
    Json(json!({ "status": "running" }))
}

async fn get_positions() -> Json<serde_json::Value> {
    Json(json!({ "positions": [] }))
}

async fn get_balance() -> Json<serde_json::Value> {
    Json(json!({ "sol": 12.84 }))
}

async fn get_screening() -> Json<serde_json::Value> {
    Json(json!({ "candidates": [] }))
}