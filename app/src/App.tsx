// SPDX-License-Identifier: AGPL-3.0-or-later
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface WorldSummary {
  folder_name: string;
  level_name: string;
}

function App() {
  const [worlds, setWorlds] = useState<WorldSummary[]>([]);
  const [loading, setLoading] = useState(false);

  async function loadWorlds() {
    setLoading(true);
    try {
      const result = await invoke<WorldSummary[]>("list_worlds");
      setWorlds(result);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => { loadWorlds(); }, []);

  return (
    <main className="flex min-h-screen flex-col items-center justify-center gap-4 bg-slate-950 text-slate-100 p-8">
      <h1 className="text-3xl font-semibold">Blockoria</h1>
      <button
        className="rounded-md bg-emerald-600 px-6 py-3 hover:bg-emerald-500 disabled:opacity-50"
        onClick={loadWorlds}
        disabled={loading}
      >
        {loading ? "Carregando..." : "Listar mundos"}
      </button>
      <ul className="w-full max-w-md space-y-2">
        {worlds.length === 0 ? (
          <li className="text-slate-500 text-center py-4">Nenhum mundo encontrado</li>
        ) : (
          worlds.map((w) => (
            <li key={w.folder_name} className="rounded-lg bg-slate-900 p-4 border border-slate-800">
              <p className="font-medium">{w.level_name}</p>
              <p className="text-xs text-slate-500 font-mono">{w.folder_name}</p>
            </li>
          ))
        )}
      </ul>
    </main>
  );
}

export default App;
