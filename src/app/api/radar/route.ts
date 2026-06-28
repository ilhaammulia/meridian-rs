import { NextResponse } from 'next/server';

// Direct Meteora pool-discovery fallback so the chart grid can populate even
// when the Rust backend isn't running (preview) or has no candidates yet.
const DISCOVERY = 'https://pool-discovery-api.datapi.meteora.ag/pools?page_size=12&timeframe=1h&category=trending';
const SOL_MINT = 'So11111111111111111111111111111111111111112';

export async function GET() {
  try {
    const response = await fetch(DISCOVERY, {
      cache: 'no-store',
      headers: { 'User-Agent': 'Mozilla/5.0 meridian-dashboard', accept: 'application/json' },
    });
    const payload = await response.json().catch(() => null);
    const pools = Array.isArray(payload?.data) ? payload.data : [];
    const candidates = pools
      .map((p: any) => {
        const x = p?.token_x ?? {};
        const y = p?.token_y ?? {};
        // base token is the non-SOL side
        const base = x?.address === SOL_MINT ? y : x;
        return { name: p?.name ?? '?', base: { mint: base?.address ?? null } };
      })
      .filter((c: any) => c.base.mint && c.base.mint !== SOL_MINT);
    return NextResponse.json({ data: { candidates } });
  } catch (error) {
    return NextResponse.json(
      { data: { candidates: [] }, error: error instanceof Error ? error.message : 'radar failed' },
      { status: 502 },
    );
  }
}
