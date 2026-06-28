'use client';

import { useEffect, useState } from 'react';
import { LineChart } from 'lucide-react';
import { ChartWidget, type ChartSlot } from '../widgets/ChartWidget';
import { cachedJson } from '../../lib/clientCache';

const SLOTS = 4;

const buildSlots = (positions: any, candidates: any): (ChartSlot | null)[] => {
  const out: ChartSlot[] = [];
  const seen = new Set<string>();

  const push = (mint?: string | null, name?: string | null, source: ChartSlot['source'] = 'candidate') => {
    if (!mint || seen.has(mint)) return;
    seen.add(mint);
    out.push({ mint, name: (name ?? 'TOKEN').toString(), source });
  };

  // Active deployed positions first (auto-detected)
  const posList = Array.isArray(positions?.data?.positions) ? positions.data.positions : [];
  for (const p of posList) {
    if (String(p?.status ?? 'active').toLowerCase() === 'closed') continue;
    push(p?.base_mint, p?.pool_name ?? p?.base_symbol, 'position');
  }

  // Fill the rest with top radar candidates
  const candList = Array.isArray(candidates?.data?.candidates)
    ? candidates.data.candidates
    : Array.isArray(candidates?.data)
      ? candidates.data
      : [];
  for (const c of candList) {
    if (out.length >= SLOTS) break;
    push(c?.base?.mint, c?.name, 'candidate');
  }

  const slots: (ChartSlot | null)[] = out.slice(0, SLOTS);
  while (slots.length < SLOTS) slots.push(null);
  return slots;
};

export const ChartGridView = () => {
  const [slots, setSlots] = useState<(ChartSlot | null)[]>([null, null, null, null]);
  const [posCount, setPosCount] = useState(0);

  useEffect(() => {
    let mounted = true;
    const load = async () => {
      try {
        const [positions, candidates] = await Promise.all([
          cachedJson<any>('/api/meridian/positions', 8_000).catch(() => null),
          cachedJson<any>('/api/meridian/candidates?limit=40', 60_000).catch(() => null),
        ]);
        if (!mounted) return;
        let next = buildSlots(positions, candidates);
        // If the backend gave us nothing to chart, fall back to the direct
        // Meteora discovery radar so the grid still populates (preview/offline).
        if (next.every((s) => s == null)) {
          const radar = await cachedJson<any>('/api/radar', 60_000).catch(() => null);
          if (!mounted) return;
          next = buildSlots(positions, radar);
        }
        setSlots(next);
        setPosCount(next.filter((s) => s?.source === 'position').length);
      } catch {
        /* keep previous slots */
      }
    };
    load();
    const t = window.setInterval(load, 15_000);
    return () => {
      mounted = false;
      window.clearInterval(t);
    };
  }, []);

  return (
    <div className="chart-grid-view">
      <header className="chart-grid-head">
        <div><LineChart size={18} /><h2>Live Charts — Bollinger %B</h2></div>
        <span>
          {posCount > 0 ? `${posCount} position${posCount > 1 ? 's' : ''} tracked` : 'no positions — showing radar'} · BB(20,2) 5m · entry %B ≥ 0.8
        </span>
      </header>
      <div className="chart-grid">
        {slots.map((slot, i) => (
          <ChartWidget key={slot?.mint ?? `empty-${i}`} slot={slot} index={i} />
        ))}
      </div>
    </div>
  );
};
