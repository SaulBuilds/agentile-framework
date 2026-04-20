"use client";
import Link from "next/link";
import { usePathname } from "next/navigation";

const sections = [
  { name: "Dashboard", href: "/", icon: "~" },
  { name: "Generation", href: "/generation", icon: "G" },
  { name: "Sessions", href: "/sessions", icon: "S" },
  { name: "Decks", href: "/decks", icon: "D" },
  { name: "Evaluations", href: "/evaluations", icon: "E" },
  { name: "Harness", href: "/harness", icon: "H" },
  { name: "Scheduler", href: "/scheduler", icon: "J" },
  { name: "Realtime", href: "/realtime", icon: "R" },
  { name: "Governance", href: "/governance", icon: "V" },
  { name: "Audit", href: "/audit", icon: "A" },
  { name: "Settings", href: "/settings", icon: "*" },
];

export default function Sidebar() {
  const pathname = usePathname();
  return (
    <nav className="w-56 shrink-0 border-r border-zinc-800 bg-zinc-950 p-4 flex flex-col gap-1 text-sm">
      <div className="text-lg font-bold text-white mb-4 px-2">Music Box</div>
      {sections.map((s) => {
        const active = s.href === "/" ? pathname === "/" : pathname.startsWith(s.href);
        return (
          <Link
            key={s.href}
            href={s.href}
            className={`flex items-center gap-2 px-2 py-1.5 rounded ${
              active ? "bg-zinc-800 text-white" : "text-zinc-400 hover:text-zinc-200 hover:bg-zinc-900"
            }`}
          >
            <span className="w-5 text-center font-mono text-xs text-zinc-500">{s.icon}</span>
            {s.name}
          </Link>
        );
      })}
    </nav>
  );
}
