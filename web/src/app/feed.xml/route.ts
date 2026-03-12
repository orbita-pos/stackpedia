const API = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001";

interface StackItem {
  id: string;
  project_name: string;
  description: string;
  category: string;
  creator_nickname: string;
  created_at: string;
}

export async function GET() {
  let items: StackItem[] = [];
  try {
    const res = await fetch(`${API}/api/stacks?limit=20&sort=new`, {
      next: { revalidate: 300 },
    });
    if (res.ok) items = await res.json();
  } catch {
    // fallback to empty feed
  }

  const siteUrl = process.env.NEXT_PUBLIC_SITE_URL || "https://stackpedia.dev";

  const itemsXml = items
    .map(
      (s) => `    <item>
      <title>${escapeXml(s.project_name)}</title>
      <description>${escapeXml(s.description)}</description>
      <link>${siteUrl}/en/stacks/${s.id}</link>
      <guid isPermaLink="true">${siteUrl}/en/stacks/${s.id}</guid>
      <pubDate>${new Date(s.created_at).toUTCString()}</pubDate>
      <author>${escapeXml(s.creator_nickname)}</author>
      <category>${escapeXml(s.category)}</category>
    </item>`
    )
    .join("\n");

  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>Stackpedia</title>
    <description>Real stacks from real projects. No theory. No tutorials.</description>
    <link>${siteUrl}</link>
    <atom:link href="${siteUrl}/feed.xml" rel="self" type="application/rss+xml"/>
    <language>en</language>
${itemsXml}
  </channel>
</rss>`;

  return new Response(xml, {
    headers: {
      "Content-Type": "application/rss+xml; charset=utf-8",
      "Cache-Control": "public, max-age=300",
    },
  });
}

function escapeXml(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&apos;");
}
