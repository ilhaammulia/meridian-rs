'use client';

import { useState } from 'react';
import bs58 from 'bs58';
import { Lock, Wallet } from 'lucide-react';
import { Clock } from '../layout/Clock';

type PhantomProvider = {
  isPhantom?: boolean;
  connect: () => Promise<{ publicKey: { toString: () => string } }>;
  signMessage: (message: Uint8Array, display?: string) => Promise<{ signature: Uint8Array }>;
};

const getProvider = (): PhantomProvider | null => {
  if (typeof window === 'undefined') return null;
  const anyWin = window as any;
  const p = anyWin.solana ?? anyWin.phantom?.solana;
  return p?.isPhantom ? p : p ?? null;
};

export const LockScreen = ({ onAuthed }: { onAuthed: (pubkey: string) => void }) => {
  const [status, setStatus] = useState<'idle' | 'connecting' | 'signing' | 'verifying' | 'error'>('idle');
  const [error, setError] = useState('');

  const signIn = async () => {
    setError('');
    const provider = getProvider();
    if (!provider) {
      setError('Phantom wallet not found — install it to sign in.');
      setStatus('error');
      return;
    }
    try {
      setStatus('connecting');
      const { publicKey } = await provider.connect();
      const pubkey = publicKey.toString();

      const nonceRes = await fetch(`/api/auth/nonce?pubkey=${encodeURIComponent(pubkey)}`);
      const nonceData = await nonceRes.json();
      if (!nonceRes.ok) throw new Error(nonceData?.error ?? 'failed to get nonce');

      setStatus('signing');
      const signed = await provider.signMessage(new TextEncoder().encode(nonceData.message), 'utf8');
      const signature = bs58.encode(signed.signature);

      setStatus('verifying');
      const verifyRes = await fetch('/api/auth/verify', {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ pubkey, signature }),
      });
      const verifyData = await verifyRes.json();
      if (!verifyRes.ok) throw new Error(verifyData?.error ?? 'verification failed');

      onAuthed(pubkey);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'sign-in failed');
      setStatus('error');
    }
  };

  const busy = status === 'connecting' || status === 'signing' || status === 'verifying';
  const label =
    status === 'connecting' ? 'Connecting…'
      : status === 'signing' ? 'Sign in your wallet…'
        : status === 'verifying' ? 'Verifying…'
          : 'Connect wallet to unlock';

  return (
    <div className="lock-screen">
      <div className="lock-clock"><Clock type="time" /><span><Clock type="date" /></span></div>
      <div className="lock-card">
        <div className="lock-avatar"><img src="/profile-avatar.png" alt="0xRapzz" /></div>
        <h1>0xRapzz</h1>
        <p className="lock-sub">Meridian DLMM Agent</p>
        <button type="button" className="lock-btn" onClick={signIn} disabled={busy}>
          {busy ? <Lock size={16} /> : <Wallet size={16} />} {label}
        </button>
        {error ? <p className="lock-error">{error}</p> : <p className="lock-hint">Sign-In with Solana — no password, no gas</p>}
      </div>
    </div>
  );
};
