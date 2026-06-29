'use client';

import { useEffect, useState } from 'react';
import { Cpu } from 'lucide-react';
import { GlassCard } from '../ui/GlassCard';
import { cachedJson } from '../../lib/clientCache';

type ApiPayload<T = any> = { success?: boolean; data?: T; error?: string };

const Field = ({ label, value }: { label: string; value: unknown }) => (
  <div className="backend-kv">
    <span>{label}</span>
    <strong title={String(value ?? '-')}>{String(value ?? '-')}</strong>
  </div>
);

export const BackendStatusWidget = () => {
  const [status, setStatus] = useState<any>();

  useEffect(() => {
    let mounted = true;
    const load = async () => {
      const payload = await cachedJson<ApiPayload>('/api/meridian/status', 8_000).catch(() => undefined);
      if (mounted) setStatus(payload?.data);
    };
    load();
    const timer = window.setInterval(load, 10_000);
    return () => { mounted = false; window.clearInterval(timer); };
  }, []);

  return (
    <GlassCard className="backend-card backend-status-card">
      <div className="terminal-title"><Cpu size={18} />BACKEND STATUS</div>
      <div className="terminal-divider" />
      <div className="backend-status-strip">
        <b>{status?.status ?? 'loading'}</b>
        <span>{status?.dry_run ? 'DRY RUN' : 'LIVE'}</span>
      </div>
      <div className="backend-grid-two">
        <Field label="Active positions" value={status?.active_positions ?? 0} />
        <Field label="Screen every" value={`${status?.schedule?.screeningIntervalMin ?? '-'} min`} />
        <Field label="Manage every" value={`${status?.schedule?.managementIntervalMin ?? '-'} min`} />
        <Field label="PnL poll" value={`${status?.schedule?.pnlPollIntervalSecs ?? '-'} sec`} />
        <Field label="State" value={status?.state_path ? 'connected' : 'not set'} />
        <Field label="Data dir" value={status?.data_dir ? 'available' : 'unknown'} />
      </div>
    </GlassCard>
  );
};



