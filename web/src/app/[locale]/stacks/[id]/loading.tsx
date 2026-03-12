export default function Loading() {
  return (
    <div className="mx-auto max-w-3xl px-4 pt-8 pb-16">
      <div className="animate-pulse">
        {/* Header */}
        <div className="h-8 w-64 bg-[#1a1a1a] mb-3" />
        <div className="h-4 w-full max-w-md bg-[#1a1a1a] mb-2" />
        <div className="h-3 w-32 bg-[#1a1a1a] mb-8" />

        {/* Tools */}
        <div className="h-4 w-20 bg-[#1a1a1a] mb-3" />
        {Array.from({ length: 4 }).map((_, i) => (
          <div key={i} className="h-16 bg-[#1a1a1a] mb-px" />
        ))}

        {/* Lessons */}
        <div className="h-4 w-24 bg-[#1a1a1a] mt-8 mb-3" />
        <div className="h-20 bg-[#1a1a1a]" />

        {/* Comments */}
        <div className="h-4 w-28 bg-[#1a1a1a] mt-8 mb-3" />
        <div className="h-24 bg-[#1a1a1a] mb-4" />
        {Array.from({ length: 3 }).map((_, i) => (
          <div key={i} className="h-14 bg-[#1a1a1a] mb-px" />
        ))}
      </div>
    </div>
  );
}
