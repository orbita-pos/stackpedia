export default function Loading() {
  return (
    <div className="mx-auto max-w-3xl px-4 pt-8 pb-16">
      <div className="animate-pulse">
        {/* Header */}
        <div className="h-8 w-48 bg-[#1a1a1a] mb-3" />
        <div className="h-4 w-32 bg-[#1a1a1a] mb-8" />

        {/* Verdict bar */}
        <div className="h-3 w-full bg-[#1a1a1a] mb-2" />
        <div className="h-3 w-full bg-[#1a1a1a] mb-8" />

        {/* Entries */}
        <div className="h-4 w-36 bg-[#1a1a1a] mb-3" />
        {Array.from({ length: 4 }).map((_, i) => (
          <div key={i} className="h-20 bg-[#1a1a1a] mb-px" />
        ))}
      </div>
    </div>
  );
}
