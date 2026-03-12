import Link from "next/link";

export default function NotFound() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center font-mono bg-white dark:bg-[#0a0a0a] px-4">
      <div className="text-center">
        <div className="mb-6 flex items-center justify-center gap-2">
          <img src="/logo-96x96.png" alt="" width={32} height={32} />
          <span className="text-xl font-bold text-neutral-900 dark:text-white">
            stackpedia<span className="text-orange-500">.</span>
          </span>
        </div>
        <h1 className="text-6xl font-bold text-neutral-900 dark:text-white mb-2">
          404
        </h1>
        <p className="text-neutral-500 mb-8">page not found</p>
        <Link
          href="/"
          className="border border-neutral-200 dark:border-neutral-800 px-4 py-2 text-sm text-neutral-600 dark:text-neutral-400 hover:text-orange-500 hover:border-orange-500/50"
        >
          go home
        </Link>
      </div>
    </div>
  );
}
