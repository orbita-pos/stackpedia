export default function Loading() {
  return (
    <div className="mx-auto max-w-3xl px-4 pt-8 pb-16">
      <div className="animate-pulse">
        {/* Hero */}
        <div className="h-10 w-72 bg-[#1a1a1a] mb-4" />
        <div className="h-5 w-96 bg-[#1a1a1a] mb-2" />
        <div className="h-5 w-64 bg-[#1a1a1a] mb-8" />

        {/* Stats */}
        <div className="flex gap-8 mb-12">
          {Array.from({ length: 3 }).map((_, i) => (
            <div key={i} className="h-12 w-20 bg-[#1a1a1a]" />
          ))}
        </div>

        {/* Featured stack */}
        <div className="h-4 w-32 bg-[#1a1a1a] mb-3" />
        <div className="h-32 w-full bg-[#1a1a1a] mb-12" />

        {/* Trending tools */}
        <div className="h-4 w-28 bg-[#1a1a1a] mb-3" />
        <div className="flex gap-2 mb-12">
          {Array.from({ length: 5 }).map((_, i) => (
            <div key={i} className="h-8 w-24 bg-[#1a1a1a]" />
          ))}
        </div>

        {/* Latest stacks */}
        <div className="h-4 w-28 bg-[#1a1a1a] mb-3" />
        {Array.from({ length: 3 }).map((_, i) => (
          <div key={i} className="h-20 w-full bg-[#1a1a1a] mb-px" />
        ))}
      </div>
    </div>
  );
}
