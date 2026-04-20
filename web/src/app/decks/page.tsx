"use client";
import ListPanel from "@/components/ListPanel";
import ToolPanel from "@/components/ToolPanel";

export default function DecksPage() {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Decks</h1>
      <div className="flex flex-col gap-4">
        <ListPanel tool="deck_list" title="Decks" />
        <ToolPanel
          tool="deck_create"
          title="Create Deck"
          fields={[
            { name: "display_name", label: "Name", placeholder: "Deck A" },
            { name: "session_id", label: "Session ID", placeholder: "session-..." },
          ]}
        />
        <ToolPanel
          tool="deck_transport"
          title="Inspect Transport"
          fields={[{ name: "deck_id", label: "Deck ID", placeholder: "deck-..." }]}
        />
      </div>
    </div>
  );
}
