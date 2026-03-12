use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;

use crate::db::models::ErrorResponse;
use crate::db::queries;
use crate::AppState;

type ApiError = (StatusCode, Json<ErrorResponse>);

fn err(status: StatusCode, msg: &str) -> ApiError {
    (status, Json(ErrorResponse { error: msg.into() }))
}

fn hash_code(code: &str) -> Result<String, StatusCode> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(code.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

struct StackSeed {
    project_name: &'static str,
    description: &'static str,
    category: &'static str,
    scale: &'static str,
    lessons: &'static str,
    creator: usize,
    tools: &'static [ToolData],
}

struct ToolData {
    name: &'static str,
    category: &'static str,
    verdict: &'static str,
    cost: Option<&'static str>,
    why: &'static str,
}

struct CommentSeed {
    stack: usize,
    user: usize,
    text: &'static str,
}

// Votes: (stack_index, &[voter_user_indices])
struct VoteSeed {
    stack: usize,
    voters: &'static [usize],
}

const USERS: &[&str] = &[
    "marcelo", "skylar", "devjin", "rustacean42", "indie_hacker",
    "kaitlyn_dev", "0xdaniel", "priya_codes", "fullstack_frank", "lucia_mx",
    "byte_sized", "craftsman_ken", "nova_eng", "terraform_tina", "shipfast_sam",
    "arch_andrea", "deploy_dan", "pixel_paula", "rails_raja", "cloud_chloe",
    "go_gordon", "react_rita", "infra_ivan", "data_diana", "security_seth",
    "mobile_mia", "backend_boris", "frontend_fey", "oss_olivia", "scale_sergio",
    "api_alex", "devops_dani", "ml_marcos", "startup_steph", "cli_carlos",
    "elixir_eli", "vue_victor", "svelte_sara", "docker_dave", "k8s_kim",
];

const STACKS: &[StackSeed] = &[
    // ===== SaaS =====
    StackSeed {
        project_name: "Orbita POS",
        description: "Point-of-sale system for restaurants and retail with real-time inventory and payments.",
        category: "saas", scale: "thousands", creator: 0,
        lessons: "Should have invested in error monitoring from day one. Lost 2 days debugging a prod issue Sentry would have caught in 5 minutes.",
        tools: &[
            ToolData { name: "next.js", category: "frontend", verdict: "love", cost: Some("$0"), why: "Fullstack React with SSR. Nothing else comes close for developer productivity" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "Rock solid. Never had a single issue in 2 years" },
            ToolData { name: "vercel", category: "hosting", verdict: "good", cost: Some("$20/mo"), why: "Great DX but costs add up fast once you leave hobby tier" },
            ToolData { name: "stripe", category: "payments", verdict: "love", cost: Some("2.9%+30c"), why: "Best payment API ever made. Documentation is perfect" },
            ToolData { name: "tailwindcss", category: "frontend", verdict: "love", cost: Some("$0"), why: "Can't imagine writing CSS any other way. Ship UI 10x faster" },
            ToolData { name: "resend", category: "email", verdict: "good", cost: Some("$20/mo"), why: "Clean API, reliable delivery. Replaced SendGrid and never looked back" },
        ],
    },
    StackSeed {
        project_name: "Webhook Relay",
        description: "Infrastructure service that receives, queues, and reliably delivers webhooks with retry logic.",
        category: "api", scale: "tens_of_thousands", creator: 1,
        lessons: "Started with PostgreSQL but SQLite was 10x simpler for our use case. Don't over-engineer your database.",
        tools: &[
            ToolData { name: "go", category: "backend", verdict: "love", cost: Some("$0"), why: "Fast compilation, small binaries, incredible standard library" },
            ToolData { name: "sqlite", category: "database", verdict: "good", cost: Some("$0"), why: "Embedded, zero config. WAL mode handles our read-heavy workload perfectly" },
            ToolData { name: "railway", category: "hosting", verdict: "good", cost: Some("$5/mo"), why: "Deploy from Git push. Simple pricing. Cold starts on free tier though" },
            ToolData { name: "redis", category: "database", verdict: "good", cost: Some("$0"), why: "Rate limiting and queue. Bulletproof" },
            ToolData { name: "prometheus", category: "monitoring", verdict: "love", cost: Some("$0"), why: "Best-in-class metrics. Paired with Grafana it's unbeatable" },
        ],
    },
    StackSeed {
        project_name: "FitTrack",
        description: "Mobile fitness tracking app with workout logging, progress photos, and social challenges.",
        category: "mobile", scale: "hundreds", creator: 2,
        lessons: "Expo Go is amazing for dev but you'll hit its limits fast with native modules. Switch to dev builds early.",
        tools: &[
            ToolData { name: "react native", category: "frontend", verdict: "good", cost: Some("$0"), why: "Write once, run everywhere mostly works. Some native modules are painful" },
            ToolData { name: "supabase", category: "database", verdict: "love", cost: Some("$25/mo"), why: "Firebase killer. Postgres under the hood, real-time subscriptions, auth built in" },
            ToolData { name: "expo", category: "frontend", verdict: "love", cost: Some("$0"), why: "Makes React Native bearable. EAS Build is a game changer" },
            ToolData { name: "cloudflare r2", category: "storage", verdict: "love", cost: Some("$0"), why: "S3-compatible with zero egress fees. No-brainer for image storage" },
            ToolData { name: "sentry", category: "monitoring", verdict: "good", cost: Some("$26/mo"), why: "Crash reporting that actually works. Worth every penny for mobile" },
        ],
    },
    StackSeed {
        project_name: "gitkv",
        description: "CLI tool that turns any Git repo into a key-value store. Version-controlled config management.",
        category: "devtool", scale: "hobby", creator: 3,
        lessons: "Spent too long making it async when synchronous code would have been simpler. Not everything needs tokio.",
        tools: &[
            ToolData { name: "rust", category: "backend", verdict: "love", cost: Some("$0"), why: "If it compiles, it works. The type system catches bugs before they happen" },
            ToolData { name: "sqlite", category: "database", verdict: "love", cost: Some("$0"), why: "Single file database. Perfect for CLI tools that need local state" },
            ToolData { name: "clap", category: "backend", verdict: "love", cost: Some("$0"), why: "Best CLI argument parser in any language. Derive macros make it effortless" },
            ToolData { name: "github actions", category: "hosting", verdict: "good", cost: Some("$0"), why: "Free CI/CD for open source. YAML is annoying but it works" },
        ],
    },
    StackSeed {
        project_name: "ShopFlow",
        description: "Shopify storefront with custom checkout, inventory sync, and analytics dashboard.",
        category: "ecommerce", scale: "thousands", creator: 4,
        lessons: "Don't build on a platform's free tier if your business depends on it. PlanetScale removing theirs cost us a week.",
        tools: &[
            ToolData { name: "next.js", category: "frontend", verdict: "good", cost: Some("$0"), why: "App Router is powerful but the learning curve is steep" },
            ToolData { name: "shopify api", category: "backend", verdict: "meh", cost: Some("$29/mo"), why: "API is comprehensive but rate limits are aggressive and docs inconsistent" },
            ToolData { name: "planetscale", category: "database", verdict: "regret", cost: Some("$29/mo"), why: "Great product but they killed the free tier. Migrating to Turso now" },
            ToolData { name: "vercel", category: "hosting", verdict: "good", cost: Some("$20/mo"), why: "Perfect for Next.js but vendor lock-in is real" },
            ToolData { name: "clerk", category: "auth", verdict: "love", cost: Some("$25/mo"), why: "Best auth DX I've ever used. Saved weeks of development time" },
            ToolData { name: "stripe", category: "payments", verdict: "love", cost: Some("2.9%+30c"), why: "Stripe is the gold standard. Checkout and billing portal save massive time" },
        ],
    },
    // 5
    StackSeed {
        project_name: "InvoiceNinja Clone",
        description: "Open-source invoicing platform with recurring billing, expense tracking, and multi-currency support.",
        category: "saas", scale: "hundreds", creator: 5,
        lessons: "Multi-currency is 10x harder than you think. Use a library like dinero.js from day one, don't roll your own.",
        tools: &[
            ToolData { name: "laravel", category: "backend", verdict: "love", cost: Some("$0"), why: "The ecosystem is unmatched. Eloquent ORM, queues, scheduling — all built in" },
            ToolData { name: "vue", category: "frontend", verdict: "love", cost: Some("$0"), why: "Simpler mental model than React. Composition API is elegant" },
            ToolData { name: "mysql", category: "database", verdict: "good", cost: Some("$0"), why: "Works fine for our scale. Would use Postgres if starting fresh" },
            ToolData { name: "digitalocean", category: "hosting", verdict: "good", cost: Some("$12/mo"), why: "Simple, predictable pricing. App Platform makes deploys easy" },
            ToolData { name: "stripe", category: "payments", verdict: "love", cost: Some("2.9%+30c"), why: "Invoicing API is exactly what we needed. Webhooks are reliable" },
        ],
    },
    // 6
    StackSeed {
        project_name: "TeamSync",
        description: "Project management tool for remote teams with real-time boards, time tracking, and standup automation.",
        category: "saas", scale: "tens_of_thousands", creator: 6,
        lessons: "Real-time collaboration is the hardest feature you'll ever build. Use CRDTs or an existing solution like Liveblocks, don't DIY.",
        tools: &[
            ToolData { name: "next.js", category: "frontend", verdict: "love", cost: Some("$0"), why: "Server components changed the game for us. Data fetching is so clean now" },
            ToolData { name: "prisma", category: "database", verdict: "good", cost: Some("$0"), why: "Type-safe queries are great. Migrations work well. Performance OK for our scale" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "JSONB columns saved us from needing a separate document store" },
            ToolData { name: "vercel", category: "hosting", verdict: "good", cost: Some("$20/mo"), why: "Edge functions are fast. Preview deployments are invaluable for our team" },
            ToolData { name: "clerk", category: "auth", verdict: "love", cost: Some("$25/mo"), why: "Organization support is perfect for our multi-tenant setup" },
            ToolData { name: "liveblocks", category: "backend", verdict: "love", cost: Some("$99/mo"), why: "Real-time presence and collaboration without building it ourselves" },
            ToolData { name: "resend", category: "email", verdict: "love", cost: Some("$20/mo"), why: "React Email templates are incredible. Our transactional emails look professional" },
        ],
    },
    // 7
    StackSeed {
        project_name: "MetricsDash",
        description: "Business analytics dashboard that aggregates data from Stripe, GA4, and social media into one view.",
        category: "saas", scale: "hundreds", creator: 7,
        lessons: "Don't build your own charting library. Recharts + shadcn/ui charts saved months of work.",
        tools: &[
            ToolData { name: "next.js", category: "frontend", verdict: "love", cost: Some("$0"), why: "API routes + frontend in one project. Monorepo without the complexity" },
            ToolData { name: "shadcn/ui", category: "frontend", verdict: "love", cost: Some("$0"), why: "Copy-paste components you actually own. Perfect balance of flexibility and speed" },
            ToolData { name: "supabase", category: "database", verdict: "love", cost: Some("$25/mo"), why: "Row-level security handles multi-tenancy beautifully" },
            ToolData { name: "vercel", category: "hosting", verdict: "good", cost: Some("$20/mo"), why: "Cron jobs for data sync are convenient. ISR for dashboard pages" },
            ToolData { name: "tailwindcss", category: "frontend", verdict: "love", cost: Some("$0"), why: "With shadcn/ui it's the perfect combo. Dark mode was trivial" },
        ],
    },
    // 8
    StackSeed {
        project_name: "HelpBridge",
        description: "Customer support platform with live chat, ticket management, and AI-powered response suggestions.",
        category: "saas", scale: "thousands", creator: 8,
        lessons: "WebSocket connections at scale are expensive. Use server-sent events where you can — most chat UIs only need one-way real-time.",
        tools: &[
            ToolData { name: "react", category: "frontend", verdict: "good", cost: Some("$0"), why: "Ecosystem is huge. Finding developers is easy. But hook dependencies are footguns" },
            ToolData { name: "node.js", category: "backend", verdict: "good", cost: Some("$0"), why: "JavaScript everywhere. Sharing validation logic between client and server is great" },
            ToolData { name: "mongodb", category: "database", verdict: "meh", cost: Some("$57/mo"), why: "Flexible schema was good at first but became a nightmare. Wish we started with Postgres" },
            ToolData { name: "aws", category: "hosting", verdict: "good", cost: Some("$200/mo"), why: "Everything you need but the console is a maze. Costs are unpredictable" },
            ToolData { name: "openai api", category: "backend", verdict: "love", cost: Some("$50/mo"), why: "GPT-4 for response suggestions is magic. Customers think our agents are geniuses" },
            ToolData { name: "redis", category: "database", verdict: "love", cost: Some("$0"), why: "Pub/sub for real-time chat is perfect. Also use it for session storage" },
        ],
    },
    // 9
    StackSeed {
        project_name: "CalSync",
        description: "Scheduling tool that syncs across Google, Outlook, and Apple calendars with smart conflict resolution.",
        category: "saas", scale: "tens_of_thousands", creator: 9,
        lessons: "Calendar APIs are the worst APIs in existence. Google Calendar API rate limits will make you cry. Cache aggressively.",
        tools: &[
            ToolData { name: "typescript", category: "backend", verdict: "love", cost: Some("$0"), why: "Strict mode catches so many bugs. Worth the initial setup pain" },
            ToolData { name: "fastify", category: "backend", verdict: "love", cost: Some("$0"), why: "5x faster than Express. Plugin system is well designed. JSON schema validation built in" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "Date/time handling is excellent. Range types for calendar slots are perfect" },
            ToolData { name: "redis", category: "database", verdict: "love", cost: Some("$0"), why: "Caching API responses. Reduced our Google Calendar API calls by 90%" },
            ToolData { name: "fly.io", category: "hosting", verdict: "love", cost: Some("$10/mo"), why: "Multi-region deployment was critical for us. Latency matters for sync" },
            ToolData { name: "resend", category: "email", verdict: "good", cost: Some("$20/mo"), why: "Calendar invite emails need to be reliable. Resend delivers" },
        ],
    },
    // 10
    StackSeed {
        project_name: "SurveyStack",
        description: "Form builder and survey platform with branching logic, analytics, and embeddable widgets.",
        category: "saas", scale: "hundreds", creator: 10,
        lessons: "Form validation on both client and server is non-negotiable. We shipped with client-only validation and got garbage data for a month.",
        tools: &[
            ToolData { name: "svelte", category: "frontend", verdict: "love", cost: Some("$0"), why: "No virtual DOM overhead. Forms are buttery smooth. Reactive statements are beautiful" },
            ToolData { name: "sveltekit", category: "frontend", verdict: "love", cost: Some("$0"), why: "Form actions and load functions are elegant. Best DX of any meta-framework" },
            ToolData { name: "turso", category: "database", verdict: "love", cost: Some("$0"), why: "SQLite at the edge. Perfect for our read-heavy survey results pages" },
            ToolData { name: "drizzle", category: "database", verdict: "love", cost: Some("$0"), why: "SQL-like syntax, full type safety. Lighter than Prisma, no code generation" },
            ToolData { name: "cloudflare pages", category: "hosting", verdict: "love", cost: Some("$0"), why: "Free tier is incredibly generous. Workers for API routes" },
        ],
    },
    // 11
    StackSeed {
        project_name: "SubTracker",
        description: "Subscription management SaaS that tracks MRR, churn, and sends dunning emails automatically.",
        category: "saas", scale: "hundreds", creator: 11,
        lessons: "Dunning email timing matters more than content. We A/B tested and found that sending 3 days before expiry recovered 40% more subscriptions.",
        tools: &[
            ToolData { name: "remix", category: "frontend", verdict: "good", cost: Some("$0"), why: "Web fundamentals approach is refreshing. Nested routes are powerful. Smaller community though" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "Financial data needs ACID. Postgres never lets us down" },
            ToolData { name: "stripe", category: "payments", verdict: "love", cost: Some("2.9%+30c"), why: "Billing portal handles 80% of what we'd have to build ourselves" },
            ToolData { name: "fly.io", category: "hosting", verdict: "good", cost: Some("$15/mo"), why: "Docker-based deployment. Good for non-Node backends" },
            ToolData { name: "postmark", category: "email", verdict: "love", cost: Some("$15/mo"), why: "Deliverability is the best we've tested. For transactional email, nothing beats it" },
        ],
    },
    // 12
    StackSeed {
        project_name: "RecruiterAI",
        description: "AI-powered recruiting platform that screens resumes, generates interview questions, and manages pipelines.",
        category: "saas", scale: "thousands", creator: 12,
        lessons: "AI hallucinations in HR context are dangerous. Always have a human review step. We almost sent offensive auto-generated interview questions.",
        tools: &[
            ToolData { name: "python", category: "backend", verdict: "love", cost: Some("$0"), why: "AI/ML ecosystem is unmatched. LangChain + FastAPI is our golden stack" },
            ToolData { name: "fastapi", category: "backend", verdict: "love", cost: Some("$0"), why: "Type hints, auto-docs, async support. Best Python web framework by far" },
            ToolData { name: "react", category: "frontend", verdict: "good", cost: Some("$0"), why: "Huge component ecosystem. Finding React devs for hiring is easy" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "pgvector extension for semantic search is incredible" },
            ToolData { name: "openai api", category: "backend", verdict: "good", cost: Some("$200/mo"), why: "GPT-4 quality is great. Costs are high at scale. We cache heavily" },
            ToolData { name: "aws", category: "hosting", verdict: "meh", cost: Some("$150/mo"), why: "Lambda cold starts kill UX. Moved to ECS and it's better but complex" },
            ToolData { name: "auth0", category: "auth", verdict: "good", cost: Some("$23/mo"), why: "Enterprise SSO support is important for our B2B customers" },
        ],
    },
    // 13 - Ecommerce
    StackSeed {
        project_name: "CrateJoy Clone",
        description: "Subscription box marketplace where creators sell monthly curated boxes to subscribers.",
        category: "ecommerce", scale: "hundreds", creator: 13,
        lessons: "Shipping cost estimation is the hardest part of subscription boxes. Use EasyPost API — don't try to calculate rates yourself.",
        tools: &[
            ToolData { name: "ruby on rails", category: "backend", verdict: "love", cost: Some("$0"), why: "Convention over configuration saves so much time. Hotwire is underrated" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "Reliable. Rails migrations + Postgres is a proven combo" },
            ToolData { name: "stripe", category: "payments", verdict: "love", cost: Some("2.9%+30c"), why: "Subscription billing with metered usage. Customer portal is a lifesaver" },
            ToolData { name: "heroku", category: "hosting", verdict: "meh", cost: Some("$25/mo"), why: "Easy but expensive for what you get. Considering Render or Railway" },
            ToolData { name: "tailwindcss", category: "frontend", verdict: "love", cost: Some("$0"), why: "Responsive design without writing a single media query from scratch" },
            ToolData { name: "aws s3", category: "storage", verdict: "good", cost: Some("$5/mo"), why: "ActiveStorage + S3 just works. Product image management is painless" },
        ],
    },
    // 14
    StackSeed {
        project_name: "ArtisanMart",
        description: "Marketplace for handmade goods. Like Etsy but focused on Latin American artisans.",
        category: "ecommerce", scale: "thousands", creator: 14,
        lessons: "International payments are a nightmare. MercadoPago for LATAM + Stripe for US/EU. Don't try to unify them — embrace the complexity.",
        tools: &[
            ToolData { name: "nuxt", category: "frontend", verdict: "love", cost: Some("$0"), why: "Vue's meta-framework. SEO with SSR was critical for marketplace discovery" },
            ToolData { name: "vue", category: "frontend", verdict: "love", cost: Some("$0"), why: "Template syntax is intuitive. New devs on the team get productive in days" },
            ToolData { name: "django", category: "backend", verdict: "love", cost: Some("$0"), why: "Admin panel alone saved months. ORM is solid. Auth is built in" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "Full-text search with unaccented indexes for Spanish product names" },
            ToolData { name: "cloudflare", category: "cdn", verdict: "love", cost: Some("$0"), why: "Free tier handles all our static assets. Image optimization is great" },
            ToolData { name: "stripe", category: "payments", verdict: "good", cost: Some("2.9%+30c"), why: "Connect for marketplace payouts works well. Onboarding flow is smooth" },
        ],
    },
    // 15
    StackSeed {
        project_name: "DigitalVault",
        description: "Platform for selling digital products — ebooks, courses, templates, and design assets.",
        category: "ecommerce", scale: "tens_of_thousands", creator: 15,
        lessons: "Content piracy is inevitable. Watermarking and download limits are enough. DRM just punishes paying customers.",
        tools: &[
            ToolData { name: "next.js", category: "frontend", verdict: "love", cost: Some("$0"), why: "ISR for product pages is perfect. Static at scale, fresh when needed" },
            ToolData { name: "neon", category: "database", verdict: "love", cost: Some("$0"), why: "Serverless Postgres that scales to zero. Branching for preview environments is killer" },
            ToolData { name: "lemonsqueezy", category: "payments", verdict: "love", cost: Some("5%+50c"), why: "Handles VAT/sales tax globally. No need for tax compliance headaches" },
            ToolData { name: "cloudflare r2", category: "storage", verdict: "love", cost: Some("$0"), why: "No egress fees for digital downloads. Saves us thousands per month vs S3" },
            ToolData { name: "vercel", category: "hosting", verdict: "good", cost: Some("$20/mo"), why: "Edge middleware for download auth. Fast globally" },
            ToolData { name: "tailwindcss", category: "frontend", verdict: "love", cost: Some("$0"), why: "Landing pages look professional. Our conversion rate doubled after redesign" },
        ],
    },
    // 16
    StackSeed {
        project_name: "PrintDrop",
        description: "Print-on-demand storefront with design editor, mockup generator, and automatic fulfillment.",
        category: "ecommerce", scale: "hundreds", creator: 16,
        lessons: "Printful's API is slow. Cache product catalogs locally and sync every 6 hours. Real-time queries will kill your UX.",
        tools: &[
            ToolData { name: "react", category: "frontend", verdict: "good", cost: Some("$0"), why: "Canvas API integration for design editor was easier with React refs" },
            ToolData { name: "express", category: "backend", verdict: "meh", cost: Some("$0"), why: "Gets the job done but no structure. NestJS would have been smarter" },
            ToolData { name: "mongodb", category: "database", verdict: "good", cost: Some("$0"), why: "Design data is deeply nested JSON. Document model fits perfectly here" },
            ToolData { name: "railway", category: "hosting", verdict: "love", cost: Some("$10/mo"), why: "Database + server in one place. Logs are actually readable" },
            ToolData { name: "stripe", category: "payments", verdict: "love", cost: Some("2.9%+30c"), why: "Product catalog API mirrors our database. Checkout sessions are bulletproof" },
        ],
    },
    // 17 - APIs
    StackSeed {
        project_name: "PayGate",
        description: "Payment orchestration API that routes transactions to the cheapest processor based on card type and region.",
        category: "api", scale: "hundreds_of_thousands", creator: 17,
        lessons: "Never store raw card numbers, ever. PCI compliance is expensive and stressful. Use tokenization from day one.",
        tools: &[
            ToolData { name: "go", category: "backend", verdict: "love", cost: Some("$0"), why: "Goroutines for concurrent payment processing. P99 latency under 50ms" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "SERIALIZABLE isolation for payment transactions. Cannot afford race conditions" },
            ToolData { name: "redis", category: "database", verdict: "love", cost: Some("$0"), why: "Rate limiting per merchant. Lua scripts for atomic operations" },
            ToolData { name: "aws", category: "hosting", verdict: "good", cost: Some("$500/mo"), why: "Multi-AZ RDS for payment data. ECS for auto-scaling. Worth the complexity" },
            ToolData { name: "datadog", category: "monitoring", verdict: "love", cost: Some("$31/mo"), why: "APM traces for every transaction. Saved us during a processing outage" },
        ],
    },
    // 18
    StackSeed {
        project_name: "ImageKit",
        description: "Image processing API with on-the-fly resizing, format conversion, and CDN delivery.",
        category: "api", scale: "tens_of_thousands", creator: 18,
        lessons: "libvips is 10x faster than ImageMagick for web use cases. Memory usage is a fraction. Switch immediately.",
        tools: &[
            ToolData { name: "rust", category: "backend", verdict: "love", cost: Some("$0"), why: "Memory safety for image processing. No GC pauses. Throughput is insane" },
            ToolData { name: "cloudflare workers", category: "hosting", verdict: "love", cost: Some("$5/mo"), why: "Edge computing for image transformation. Sub-20ms response times globally" },
            ToolData { name: "cloudflare r2", category: "storage", verdict: "love", cost: Some("$0"), why: "Origin storage with zero egress. Our entire cost model depends on this" },
            ToolData { name: "postgresql", category: "database", verdict: "good", cost: Some("$0"), why: "Metadata and usage tracking. Overkill but we already had it running" },
            ToolData { name: "prometheus", category: "monitoring", verdict: "love", cost: Some("$0"), why: "Custom metrics for image processing times, cache hit rates, format distribution" },
        ],
    },
    // 19
    StackSeed {
        project_name: "NotifyHub",
        description: "Unified notification API — send push, email, SMS, and Slack from one API call with templates.",
        category: "api", scale: "thousands", creator: 19,
        lessons: "SMS providers are unreliable globally. Use at least 2 providers with automatic failover. Twilio isn't always the cheapest either.",
        tools: &[
            ToolData { name: "node.js", category: "backend", verdict: "good", cost: Some("$0"), why: "Async I/O is perfect for fan-out notification delivery" },
            ToolData { name: "nestjs", category: "backend", verdict: "love", cost: Some("$0"), why: "Dependency injection and modules keep the codebase sane at scale" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "Notification logs with BRIN indexes for time-series queries" },
            ToolData { name: "redis", category: "database", verdict: "love", cost: Some("$0"), why: "BullMQ for job queues. Reliable delivery with retry logic" },
            ToolData { name: "render", category: "hosting", verdict: "good", cost: Some("$25/mo"), why: "Background workers are easy to set up. Simpler than AWS for our scale" },
        ],
    },
    // 20
    StackSeed {
        project_name: "PDFForge",
        description: "API that generates beautiful PDFs from HTML templates. Invoices, reports, contracts.",
        category: "api", scale: "tens_of_thousands", creator: 20,
        lessons: "Puppeteer eats memory like crazy. Switch to Playwright — same API, better resource management. Or use wkhtmltopdf for simple layouts.",
        tools: &[
            ToolData { name: "go", category: "backend", verdict: "love", cost: Some("$0"), why: "Low memory footprint. Each worker handles hundreds of concurrent PDF jobs" },
            ToolData { name: "redis", category: "database", verdict: "love", cost: Some("$0"), why: "Job queue with priority levels. Paying customers get fast lane" },
            ToolData { name: "aws s3", category: "storage", verdict: "good", cost: Some("$10/mo"), why: "Generated PDFs stored temporarily. Lifecycle policies for auto-deletion" },
            ToolData { name: "fly.io", category: "hosting", verdict: "love", cost: Some("$20/mo"), why: "Machines API for auto-scaling PDF workers. Scale to zero when idle" },
            ToolData { name: "sentry", category: "monitoring", verdict: "good", cost: Some("$26/mo"), why: "PDF generation failures need immediate alerts. Source maps work great" },
        ],
    },
    // 21 - Mobile
    StackSeed {
        project_name: "BudgetBuddy",
        description: "Personal finance app with bank sync, spending categories, and savings goals.",
        category: "mobile", scale: "tens_of_thousands", creator: 21,
        lessons: "Plaid is essential but expensive. Negotiate pricing early. Their sandbox is excellent for development though.",
        tools: &[
            ToolData { name: "flutter", category: "frontend", verdict: "love", cost: Some("$0"), why: "One codebase, pixel-perfect UI on both platforms. Hot reload is addictive" },
            ToolData { name: "firebase", category: "database", verdict: "good", cost: Some("$25/mo"), why: "Firestore for user data, Auth for login, Cloud Functions for bank sync" },
            ToolData { name: "plaid", category: "backend", verdict: "good", cost: Some("$500/mo"), why: "Bank connection is reliable but expensive. Essential for fintech" },
            ToolData { name: "gcp", category: "hosting", verdict: "good", cost: Some("$50/mo"), why: "Cloud Run for API. Tight integration with Firebase" },
            ToolData { name: "sentry", category: "monitoring", verdict: "love", cost: Some("$26/mo"), why: "Flutter plugin works great. Stack traces from production crashes are invaluable" },
        ],
    },
    // 22
    StackSeed {
        project_name: "MealPrep",
        description: "Meal planning app with recipe discovery, grocery lists, and nutritional tracking.",
        category: "mobile", scale: "thousands", creator: 22,
        lessons: "Nutritional data APIs are surprisingly bad and expensive. We ended up scraping USDA data and hosting our own database.",
        tools: &[
            ToolData { name: "react native", category: "frontend", verdict: "good", cost: Some("$0"), why: "Code sharing with our web version. Reanimated 3 makes animations smooth" },
            ToolData { name: "expo", category: "frontend", verdict: "love", cost: Some("$0"), why: "EAS Update for OTA updates. Skip the app store review for bug fixes" },
            ToolData { name: "supabase", category: "database", verdict: "love", cost: Some("$25/mo"), why: "Postgres full-text search for recipes. Real-time for shared meal plans" },
            ToolData { name: "cloudflare r2", category: "storage", verdict: "love", cost: Some("$0"), why: "Recipe images served from edge. Fast everywhere" },
            ToolData { name: "revenucat", category: "payments", verdict: "love", cost: Some("$0"), why: "In-app subscriptions without the headache. Handles Apple and Google billing" },
        ],
    },
    // 23
    StackSeed {
        project_name: "RidePool",
        description: "Carpooling app for commuters. Matching algorithm based on routes, schedules, and preferences.",
        category: "mobile", scale: "thousands", creator: 23,
        lessons: "Google Maps API costs will surprise you. Use Mapbox for mobile — cheaper, offline maps included, better customization.",
        tools: &[
            ToolData { name: "kotlin", category: "frontend", verdict: "love", cost: Some("$0"), why: "Jetpack Compose is finally good. Native Android performance matters for maps" },
            ToolData { name: "swift", category: "frontend", verdict: "love", cost: Some("$0"), why: "SwiftUI for iOS. MapKit is free and good enough for our needs" },
            ToolData { name: "fastapi", category: "backend", verdict: "love", cost: Some("$0"), why: "Matching algorithm runs in Python. Async endpoints for real-time location" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "PostGIS for geospatial queries. Finding nearby riders is one SQL query" },
            ToolData { name: "firebase", category: "backend", verdict: "good", cost: Some("$25/mo"), why: "Push notifications and real-time location sharing. FCM is reliable" },
        ],
    },
    // 24
    StackSeed {
        project_name: "ZenFlow",
        description: "Meditation and wellness app with guided sessions, breathing exercises, and sleep stories.",
        category: "mobile", scale: "hundreds_of_thousands", creator: 24,
        lessons: "Audio streaming on mobile is harder than video. Offline downloads, background playback, interruption handling — test on real devices constantly.",
        tools: &[
            ToolData { name: "flutter", category: "frontend", verdict: "love", cost: Some("$0"), why: "Custom animations for breathing exercises. Lottie integration is seamless" },
            ToolData { name: "firebase", category: "database", verdict: "good", cost: Some("$100/mo"), why: "Remote Config for A/B testing meditation flows. Analytics built in" },
            ToolData { name: "cloudflare r2", category: "storage", verdict: "love", cost: Some("$10/mo"), why: "Audio files served globally. Cost is a fraction of what S3 would be" },
            ToolData { name: "revenucat", category: "payments", verdict: "love", cost: Some("1%"), why: "Subscription management across iOS and Android. Paywalls are easy" },
            ToolData { name: "amplitude", category: "monitoring", verdict: "good", cost: Some("$0"), why: "User behavior tracking. We learned most users quit at day 3 — added push reminders" },
        ],
    },
    // 25
    StackSeed {
        project_name: "LinguaLeap",
        description: "Language learning app with spaced repetition, voice recognition, and community challenges.",
        category: "mobile", scale: "tens_of_thousands", creator: 25,
        lessons: "Speech recognition accuracy varies wildly by language. Google's is best for European languages, but for tonal languages you need specialized models.",
        tools: &[
            ToolData { name: "react native", category: "frontend", verdict: "meh", cost: Some("$0"), why: "Hermes engine helps but still laggy on low-end Android. Considering Flutter" },
            ToolData { name: "django", category: "backend", verdict: "love", cost: Some("$0"), why: "REST framework for API. Admin panel for content management is irreplaceable" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "Spaced repetition algorithm queries are fast with proper indexes" },
            ToolData { name: "aws", category: "hosting", verdict: "good", cost: Some("$80/mo"), why: "S3 for audio, Polly for TTS, Transcribe for speech-to-text. All in one cloud" },
            ToolData { name: "sentry", category: "monitoring", verdict: "love", cost: Some("$26/mo"), why: "React Native crashes are hard to debug without this. Performance monitoring too" },
            ToolData { name: "firebase", category: "auth", verdict: "good", cost: Some("$0"), why: "Social login + anonymous auth. Migration to our own auth was too risky" },
        ],
    },
    // 26 - Desktop
    StackSeed {
        project_name: "CodeSnap",
        description: "VS Code extension that creates beautiful code screenshots with themes, fonts, and export options.",
        category: "desktop", scale: "tens_of_thousands", creator: 26,
        lessons: "VS Code extension API is poorly documented. Read other popular extensions' source code — that's the real documentation.",
        tools: &[
            ToolData { name: "typescript", category: "backend", verdict: "love", cost: Some("$0"), why: "VS Code extensions must be TypeScript. The API types save you from runtime errors" },
            ToolData { name: "react", category: "frontend", verdict: "good", cost: Some("$0"), why: "Webview UI with React. Hot reload works with esbuild" },
            ToolData { name: "esbuild", category: "frontend", verdict: "love", cost: Some("$0"), why: "Extension bundling in 200ms. webpack took 15 seconds" },
            ToolData { name: "github actions", category: "hosting", verdict: "love", cost: Some("$0"), why: "Publish to VS Code marketplace on tag push. Fully automated" },
        ],
    },
    // 27
    StackSeed {
        project_name: "NoteGraph",
        description: "Electron-based note-taking app with bidirectional links, graph visualization, and local-first storage.",
        category: "desktop", scale: "thousands", creator: 27,
        lessons: "Electron apps are RAM hogs. Tauri would halve our memory usage but the ecosystem isn't mature enough for complex UIs yet.",
        tools: &[
            ToolData { name: "electron", category: "frontend", verdict: "meh", cost: Some("$0"), why: "Cross-platform works but 200MB+ app size and high RAM usage. Users complain" },
            ToolData { name: "react", category: "frontend", verdict: "good", cost: Some("$0"), why: "Virtual scroll for large note lists. Prosemirror for rich text editing" },
            ToolData { name: "sqlite", category: "database", verdict: "love", cost: Some("$0"), why: "Local-first means SQLite. Full-text search with FTS5 is excellent" },
            ToolData { name: "tailwindcss", category: "frontend", verdict: "love", cost: Some("$0"), why: "Consistent UI across macOS, Windows, Linux. Dark mode trivial" },
            ToolData { name: "github actions", category: "hosting", verdict: "good", cost: Some("$0"), why: "Auto-builds for all 3 platforms. Code signing for macOS is painful though" },
        ],
    },
    // 28
    StackSeed {
        project_name: "DeployBot",
        description: "CLI tool for zero-downtime deployments to VPS. Like Kamal but simpler.",
        category: "devtool", scale: "hundreds", creator: 28,
        lessons: "SSH connection pooling matters. Opening a new SSH connection per command added 2 seconds per deploy step. Mux them.",
        tools: &[
            ToolData { name: "go", category: "backend", verdict: "love", cost: Some("$0"), why: "Single binary distribution. Users just download and run. No runtime needed" },
            ToolData { name: "docker", category: "backend", verdict: "love", cost: Some("$0"), why: "Container-based deploys are the only sane way. Reproducible environments" },
            ToolData { name: "github actions", category: "hosting", verdict: "love", cost: Some("$0"), why: "CI pipeline tests against real VPS. Matrix builds for linux/mac/windows" },
            ToolData { name: "sqlite", category: "database", verdict: "good", cost: Some("$0"), why: "Deployment history stored locally. Simple migrations for config schema changes" },
        ],
    },
    // 29
    StackSeed {
        project_name: "TablePlus Clone",
        description: "Open-source database GUI for PostgreSQL, MySQL, and SQLite with query editor and visual schema.",
        category: "desktop", scale: "thousands", creator: 29,
        lessons: "Tauri is ready for production desktop apps now. Our app is 8MB vs 200MB+ for Electron equivalents. Users love the speed.",
        tools: &[
            ToolData { name: "tauri", category: "frontend", verdict: "love", cost: Some("$0"), why: "Rust backend, web frontend, 8MB bundle. The future of desktop apps" },
            ToolData { name: "svelte", category: "frontend", verdict: "love", cost: Some("$0"), why: "Lightweight and fast. Perfect for Tauri's webview. No virtual DOM overhead" },
            ToolData { name: "rust", category: "backend", verdict: "good", cost: Some("$0"), why: "System-level database connections. Memory safe. Compile times are the tradeoff" },
            ToolData { name: "tailwindcss", category: "frontend", verdict: "love", cost: Some("$0"), why: "Consistent look across platforms. Custom scrollbar styling was tricky though" },
        ],
    },
    // 30 - DevTools
    StackSeed {
        project_name: "TestForge",
        description: "AI-powered test generation tool that reads your code and creates unit tests with high coverage.",
        category: "devtool", scale: "hundreds", creator: 30,
        lessons: "Generated tests that just assert 'true' are worse than no tests. Validate that tests actually fail on wrong behavior.",
        tools: &[
            ToolData { name: "python", category: "backend", verdict: "good", cost: Some("$0"), why: "AST parsing libraries are excellent. Tree-sitter bindings for multi-language support" },
            ToolData { name: "openai api", category: "backend", verdict: "good", cost: Some("$100/mo"), why: "GPT-4 generates surprisingly good tests. Function calling for structured output" },
            ToolData { name: "fastapi", category: "backend", verdict: "love", cost: Some("$0"), why: "API for CI/CD integration. WebSocket for real-time test generation progress" },
            ToolData { name: "redis", category: "database", verdict: "good", cost: Some("$0"), why: "Caching parsed ASTs. Significant speedup for re-runs on same codebase" },
            ToolData { name: "railway", category: "hosting", verdict: "good", cost: Some("$10/mo"), why: "Simple deployment for Python apps. Logs are great for debugging" },
        ],
    },
    // 31
    StackSeed {
        project_name: "StatusOwl",
        description: "Open-source status page with uptime monitoring, incident management, and subscriber notifications.",
        category: "devtool", scale: "tens_of_thousands", creator: 31,
        lessons: "Your status page infrastructure must be completely independent from your main infrastructure. If your cloud goes down, your status page goes with it.",
        tools: &[
            ToolData { name: "astro", category: "frontend", verdict: "love", cost: Some("$0"), why: "Static status pages are fast and reliable. Island architecture for interactive parts only" },
            ToolData { name: "go", category: "backend", verdict: "love", cost: Some("$0"), why: "Health check workers run with minimal resources. Single binary deployment" },
            ToolData { name: "turso", category: "database", verdict: "love", cost: Some("$0"), why: "Edge database for multi-region status checks. Embedded replicas are perfect" },
            ToolData { name: "cloudflare pages", category: "hosting", verdict: "love", cost: Some("$0"), why: "Independent from AWS/GCP. If they go down, our status page stays up" },
            ToolData { name: "betterstack", category: "monitoring", verdict: "love", cost: Some("$24/mo"), why: "We monitor our own monitoring. Inception but necessary" },
        ],
    },
    // 32
    StackSeed {
        project_name: "FlagSmith Clone",
        description: "Feature flag service with gradual rollouts, A/B testing, and real-time flag evaluation.",
        category: "devtool", scale: "thousands", creator: 32,
        lessons: "Feature flag tech debt is real. Add a mandatory expiration date to every flag. We had 200+ stale flags after a year.",
        tools: &[
            ToolData { name: "rust", category: "backend", verdict: "love", cost: Some("$0"), why: "Flag evaluation needs to be sub-millisecond. Rust delivers consistently" },
            ToolData { name: "react", category: "frontend", verdict: "good", cost: Some("$0"), why: "Dashboard for flag management. Drag-and-drop for rollout percentages" },
            ToolData { name: "redis", category: "database", verdict: "love", cost: Some("$0"), why: "Flag cache with pub/sub for instant propagation across instances" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "Audit log for who changed what flag when. Critical for compliance" },
            ToolData { name: "fly.io", category: "hosting", verdict: "love", cost: Some("$15/mo"), why: "Multi-region for low-latency flag evaluation. Machines API for auto-scaling" },
        ],
    },
    // 33
    StackSeed {
        project_name: "LogRocket Lite",
        description: "Lightweight session replay and error tracking. Privacy-first alternative to FullStory.",
        category: "devtool", scale: "hundreds", creator: 33,
        lessons: "Recording DOM mutations at 60fps generates insane amounts of data. Compression and smart sampling are essential. We went from 50MB to 2MB per session.",
        tools: &[
            ToolData { name: "typescript", category: "frontend", verdict: "love", cost: Some("$0"), why: "SDK needs rock-solid types. Customers integrate into their own TS projects" },
            ToolData { name: "go", category: "backend", verdict: "love", cost: Some("$0"), why: "Ingestion server handles thousands of events per second. Low memory footprint" },
            ToolData { name: "clickhouse", category: "database", verdict: "love", cost: Some("$0"), why: "Time-series event data. Columnar storage means queries over millions of events are fast" },
            ToolData { name: "aws s3", category: "storage", verdict: "good", cost: Some("$30/mo"), why: "Session recordings stored in S3. Lifecycle policies delete after 30 days" },
            ToolData { name: "aws", category: "hosting", verdict: "meh", cost: Some("$200/mo"), why: "ECS + ALB. Overkill for our stage but needed for HIPAA compliance" },
        ],
    },
    // 34
    StackSeed {
        project_name: "CIForge",
        description: "Self-hosted CI/CD platform with container-based builds, caching, and parallel test execution.",
        category: "devtool", scale: "hundreds", creator: 34,
        lessons: "Docker-in-Docker is fragile. Use Kaniko for building images in CI. It's rootless and doesn't need a Docker daemon.",
        tools: &[
            ToolData { name: "go", category: "backend", verdict: "love", cost: Some("$0"), why: "Orchestrating containers needs low-level control. Go's exec and io packages are perfect" },
            ToolData { name: "docker", category: "backend", verdict: "love", cost: Some("$0"), why: "Every build runs in an isolated container. Reproducible builds guaranteed" },
            ToolData { name: "postgresql", category: "database", verdict: "good", cost: Some("$0"), why: "Build logs, pipeline configs, user data. Simple and reliable" },
            ToolData { name: "redis", category: "database", verdict: "love", cost: Some("$0"), why: "Build cache with TTL. Cache hit rate of 80% cut our build times in half" },
            ToolData { name: "vue", category: "frontend", verdict: "love", cost: Some("$0"), why: "Dashboard with real-time build logs. Vue's reactivity makes streaming logs easy" },
        ],
    },
    // 35 - Other
    StackSeed {
        project_name: "Blogfolio",
        description: "Personal blog and portfolio platform for developers with MDX support and analytics.",
        category: "other", scale: "hundreds", creator: 35,
        lessons: "MDX is overkill for most blogs. Markdown + custom components with rehype plugins gives you 90% of the benefit with none of the build complexity.",
        tools: &[
            ToolData { name: "astro", category: "frontend", verdict: "love", cost: Some("$0"), why: "Content collections are perfect for blogs. Zero JS by default means fast pages" },
            ToolData { name: "tailwindcss", category: "frontend", verdict: "love", cost: Some("$0"), why: "Typography plugin for blog posts is beautiful out of the box" },
            ToolData { name: "cloudflare pages", category: "hosting", verdict: "love", cost: Some("$0"), why: "Free hosting with fast global CDN. GitHub integration for auto-deploys" },
            ToolData { name: "plausible", category: "monitoring", verdict: "love", cost: Some("$9/mo"), why: "Privacy-respecting analytics. No cookie banners needed. Simple and honest" },
        ],
    },
    // 36
    StackSeed {
        project_name: "LinkTree Clone",
        description: "Link-in-bio tool with custom themes, click analytics, and social media integration.",
        category: "other", scale: "tens_of_thousands", creator: 36,
        lessons: "Vanity URLs seem simple but are a nightmare. Reserved words, unicode handling, offensive name filtering — plan for all of it.",
        tools: &[
            ToolData { name: "sveltekit", category: "frontend", verdict: "love", cost: Some("$0"), why: "Server-side rendering for SEO. Each profile page is its own route. Fast navigation" },
            ToolData { name: "drizzle", category: "database", verdict: "love", cost: Some("$0"), why: "Type-safe queries without the Prisma overhead. Push-based migrations are simple" },
            ToolData { name: "turso", category: "database", verdict: "love", cost: Some("$0"), why: "SQLite on the edge. Profile pages load in under 100ms globally" },
            ToolData { name: "cloudflare pages", category: "hosting", verdict: "love", cost: Some("$0"), why: "Free tier handles our traffic. Worker for click tracking analytics" },
            ToolData { name: "tailwindcss", category: "frontend", verdict: "love", cost: Some("$0"), why: "CSS variables for user-customizable themes. Each profile feels unique" },
        ],
    },
    // 37
    StackSeed {
        project_name: "FileDrop",
        description: "End-to-end encrypted file sharing. Upload, get a link, share. Files auto-delete after download.",
        category: "other", scale: "thousands", creator: 37,
        lessons: "Client-side encryption is mandatory but makes debugging impossible. Log metadata (file size, type) but never content.",
        tools: &[
            ToolData { name: "hono", category: "backend", verdict: "love", cost: Some("$0"), why: "Edge-native web framework. Runs on Cloudflare Workers, Deno, Bun, Node. Future-proof" },
            ToolData { name: "cloudflare workers", category: "hosting", verdict: "love", cost: Some("$5/mo"), why: "Streaming uploads at the edge. No cold starts. Fast everywhere" },
            ToolData { name: "cloudflare r2", category: "storage", verdict: "love", cost: Some("$0"), why: "Zero egress cost for downloads. Object lifecycle for auto-deletion" },
            ToolData { name: "htmx", category: "frontend", verdict: "love", cost: Some("$0"), why: "Upload progress, download buttons — all without a build step or JS framework" },
        ],
    },
    // 38
    StackSeed {
        project_name: "ChatRoom",
        description: "Open-source real-time chat platform with rooms, threads, file sharing, and markdown support.",
        category: "other", scale: "hundreds", creator: 38,
        lessons: "WebSocket reconnection logic is critical. Users close their laptops, switch networks, go through tunnels. Handle ALL of it gracefully.",
        tools: &[
            ToolData { name: "elixir", category: "backend", verdict: "love", cost: Some("$0"), why: "Phoenix Channels handle millions of concurrent connections. BEAM VM is built for this" },
            ToolData { name: "phoenix", category: "backend", verdict: "love", cost: Some("$0"), why: "LiveView for the admin panel, Channels for chat. One framework does everything" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "Message history with proper indexes. pg_trgm for message search" },
            ToolData { name: "fly.io", category: "hosting", verdict: "love", cost: Some("$10/mo"), why: "Elixir clustering across regions works perfectly on Fly. WebSocket sticky sessions" },
            ToolData { name: "minio", category: "storage", verdict: "good", cost: Some("$0"), why: "Self-hosted S3-compatible storage for file uploads. No vendor lock-in" },
        ],
    },
    // 39
    StackSeed {
        project_name: "headlessCMS",
        description: "API-first content management system with visual editor, webhooks, and multi-language support.",
        category: "other", scale: "thousands", creator: 39,
        lessons: "Rich text editors are the hardest UI component to build. Use Tiptap/Prosemirror. Don't even think about building your own.",
        tools: &[
            ToolData { name: "next.js", category: "frontend", verdict: "good", cost: Some("$0"), why: "Admin panel with SSR. App Router makes layouts clean but server/client boundary is confusing" },
            ToolData { name: "typescript", category: "backend", verdict: "love", cost: Some("$0"), why: "Shared types between frontend editor and backend API. End-to-end type safety" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "JSONB for flexible content schemas. GIN indexes for fast queries on structured content" },
            ToolData { name: "redis", category: "database", verdict: "good", cost: Some("$0"), why: "Cache published content. Invalidation on webhook triggers" },
            ToolData { name: "vercel", category: "hosting", verdict: "good", cost: Some("$20/mo"), why: "ISR for published content pages. API routes for the CMS backend" },
        ],
    },
    // 40
    StackSeed {
        project_name: "CartaBien",
        description: "Digital menu and ordering system for restaurants. QR code to table ordering with kitchen display.",
        category: "saas", scale: "thousands", creator: 9,
        lessons: "Offline mode is mandatory for restaurants. WiFi drops during rush hour. Local-first with sync is the only reliable architecture.",
        tools: &[
            ToolData { name: "next.js", category: "frontend", verdict: "love", cost: Some("$0"), why: "PWA capabilities for offline menu. Service workers cache everything" },
            ToolData { name: "supabase", category: "database", verdict: "love", cost: Some("$25/mo"), why: "Real-time for kitchen display. Orders appear instantly without polling" },
            ToolData { name: "stripe", category: "payments", verdict: "good", cost: Some("2.9%+30c"), why: "Terminal API for in-person payments. Integration was smooth" },
            ToolData { name: "vercel", category: "hosting", verdict: "good", cost: Some("$20/mo"), why: "Fast menu loading is critical. Nobody waits 3 seconds to see a menu" },
            ToolData { name: "tailwindcss", category: "frontend", verdict: "love", cost: Some("$0"), why: "Beautiful responsive menus. Works on every phone screen size" },
        ],
    },
    // 41
    StackSeed {
        project_name: "DocuSign Lite",
        description: "Electronic signature platform with document templates, audit trails, and multi-party signing flows.",
        category: "saas", scale: "hundreds", creator: 10,
        lessons: "Legal compliance varies by country. In the EU you need qualified electronic signatures for some documents. Research before building.",
        tools: &[
            ToolData { name: "django", category: "backend", verdict: "love", cost: Some("$0"), why: "Admin panel for template management. Django REST Framework for the API" },
            ToolData { name: "react", category: "frontend", verdict: "good", cost: Some("$0"), why: "PDF.js for document rendering. Canvas API for signature capture" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "Audit trails need ACID guarantees. Row-level security for multi-tenancy" },
            ToolData { name: "aws s3", category: "storage", verdict: "good", cost: Some("$10/mo"), why: "Signed URLs for secure document access. Encryption at rest built in" },
            ToolData { name: "sendgrid", category: "email", verdict: "meh", cost: Some("$20/mo"), why: "Reliable but the API is showing its age. Template system is clunky" },
            ToolData { name: "render", category: "hosting", verdict: "good", cost: Some("$25/mo"), why: "Simple deployment for Django. Background workers for PDF generation" },
        ],
    },
    // 42
    StackSeed {
        project_name: "CryptoTracker",
        description: "Portfolio tracking dashboard for crypto with P&L calculations, tax reports, and DeFi integration.",
        category: "saas", scale: "tens_of_thousands", creator: 6,
        lessons: "Crypto APIs go down constantly. Cache everything. Have fallback providers. CoinGecko rate limits will ruin your weekend.",
        tools: &[
            ToolData { name: "next.js", category: "frontend", verdict: "good", cost: Some("$0"), why: "Charts and real-time data updates. TanStack Query for data fetching" },
            ToolData { name: "typescript", category: "backend", verdict: "love", cost: Some("$0"), why: "BigNumber types for financial calculations. Never use floating point for money" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "TimescaleDB extension for price history. Continuous aggregates for daily summaries" },
            ToolData { name: "redis", category: "database", verdict: "love", cost: Some("$0"), why: "Price cache with 60s TTL. Reduces API calls to CoinGecko by 99%" },
            ToolData { name: "vercel", category: "hosting", verdict: "good", cost: Some("$20/mo"), why: "Cron functions for price fetching every minute. Edge functions for fast API" },
            ToolData { name: "clerk", category: "auth", verdict: "love", cost: Some("$25/mo"), why: "Web3 wallet login + traditional auth. Best of both worlds" },
        ],
    },
    // 43
    StackSeed {
        project_name: "FormCraft",
        description: "No-code form builder with conditional logic, file uploads, payment collection, and zapier integration.",
        category: "saas", scale: "thousands", creator: 7,
        lessons: "Conditional logic UI is 10x harder than the engine. Use a flow-based visual editor from the start, not nested if/then dropdowns.",
        tools: &[
            ToolData { name: "svelte", category: "frontend", verdict: "love", cost: Some("$0"), why: "Drag-and-drop form builder is buttery smooth. Svelte animations are native and fast" },
            ToolData { name: "sveltekit", category: "frontend", verdict: "love", cost: Some("$0"), why: "SSR for public form pages. Embedded forms load fast for our customers' visitors" },
            ToolData { name: "turso", category: "database", verdict: "good", cost: Some("$0"), why: "Form submissions stored at the edge. Low latency for form submit responses" },
            ToolData { name: "stripe", category: "payments", verdict: "love", cost: Some("2.9%+30c"), why: "Payment fields embedded in forms. Checkout sessions created on submit" },
            ToolData { name: "cloudflare pages", category: "hosting", verdict: "love", cost: Some("$0"), why: "Public form pages served from edge. Zero cold start for form renders" },
            ToolData { name: "uploadthing", category: "storage", verdict: "love", cost: Some("$0"), why: "File upload handling without managing S3. Presigned URLs with type checking" },
        ],
    },
    // 44
    StackSeed {
        project_name: "VetCloud",
        description: "Veterinary clinic management with appointment booking, medical records, and prescription management.",
        category: "saas", scale: "hundreds", creator: 8,
        lessons: "Healthcare data regulations apply to veterinary data in some jurisdictions. Check your local laws before storing medical records in the cloud.",
        tools: &[
            ToolData { name: "ruby on rails", category: "backend", verdict: "love", cost: Some("$0"), why: "Rapid development. Active Record for complex medical record relationships" },
            ToolData { name: "react", category: "frontend", verdict: "good", cost: Some("$0"), why: "Complex forms for medical records. React Hook Form + Zod for validation" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "Medical records need strong data integrity. Postgres never loses data" },
            ToolData { name: "heroku", category: "hosting", verdict: "regret", cost: Some("$50/mo"), why: "Expensive for what we get. Moving to Railway. Heroku feels abandoned" },
            ToolData { name: "twilio", category: "email", verdict: "good", cost: Some("$20/mo"), why: "SMS appointment reminders reduce no-shows by 40%" },
        ],
    },
    // 45
    StackSeed {
        project_name: "EdgeCDN",
        description: "Open-source CDN configuration manager that deploys caching rules across Cloudflare, Fastly, and AWS.",
        category: "devtool", scale: "hobby", creator: 3,
        lessons: "Each CDN has completely different invalidation semantics. Abstract the interface but don't try to unify the behavior.",
        tools: &[
            ToolData { name: "rust", category: "backend", verdict: "love", cost: Some("$0"), why: "Concurrent API calls to multiple CDNs. Tokio async runtime is perfect" },
            ToolData { name: "clap", category: "backend", verdict: "love", cost: Some("$0"), why: "Complex CLI with subcommands. Help text generation is automatic" },
            ToolData { name: "sqlite", category: "database", verdict: "love", cost: Some("$0"), why: "Local config and deployment history. No server dependency" },
            ToolData { name: "github actions", category: "hosting", verdict: "love", cost: Some("$0"), why: "Cross-compilation for linux/mac/windows in CI. Release artifacts automatic" },
        ],
    },
    // 46
    StackSeed {
        project_name: "APIShield",
        description: "API security gateway with rate limiting, JWT validation, request signing, and abuse detection.",
        category: "api", scale: "hundreds_of_thousands", creator: 17,
        lessons: "Rate limiting by IP alone is useless. Modern attackers rotate IPs constantly. Fingerprint by behavior patterns instead.",
        tools: &[
            ToolData { name: "rust", category: "backend", verdict: "love", cost: Some("$0"), why: "Security-critical code needs memory safety. P99 latency under 1ms for auth checks" },
            ToolData { name: "redis", category: "database", verdict: "love", cost: Some("$0"), why: "Token bucket rate limiting with Lua scripts. Atomic operations are essential" },
            ToolData { name: "postgresql", category: "database", verdict: "good", cost: Some("$0"), why: "API key storage and audit logging. Not on the hot path" },
            ToolData { name: "cloudflare workers", category: "hosting", verdict: "love", cost: Some("$5/mo"), why: "Gateway must be at the edge. Sub-ms overhead per request" },
            ToolData { name: "datadog", category: "monitoring", verdict: "love", cost: Some("$31/mo"), why: "Real-time dashboards for attack detection. Custom metrics for threat patterns" },
        ],
    },
    // 47
    StackSeed {
        project_name: "MailForge",
        description: "Transactional email service with React templates, delivery tracking, and bounce management.",
        category: "api", scale: "tens_of_thousands", creator: 18,
        lessons: "Email deliverability is 80% about IP reputation and DNS config, 20% about content. Warm up new IPs slowly over 2-4 weeks.",
        tools: &[
            ToolData { name: "go", category: "backend", verdict: "love", cost: Some("$0"), why: "SMTP server in Go handles thousands of concurrent connections efficiently" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "Email events (sent, delivered, bounced, opened) with partitioned tables" },
            ToolData { name: "redis", category: "database", verdict: "love", cost: Some("$0"), why: "Queue for email sending. Retry logic with exponential backoff" },
            ToolData { name: "react", category: "frontend", verdict: "good", cost: Some("$0"), why: "React Email for templates. Customers preview in dashboard before sending" },
            ToolData { name: "aws", category: "hosting", verdict: "good", cost: Some("$100/mo"), why: "Dedicated IPs through SES. EC2 for SMTP relay. ELB for high availability" },
            ToolData { name: "clickhouse", category: "database", verdict: "love", cost: Some("$0"), why: "Analytics on millions of email events. Aggregate queries return in milliseconds" },
        ],
    },
    // 48
    StackSeed {
        project_name: "TravelBudget",
        description: "Travel planning app with budget tracking, expense splitting, and currency conversion.",
        category: "mobile", scale: "thousands", creator: 25,
        lessons: "Exchange rate APIs are expensive. Use the European Central Bank's free API and cache rates daily. Close enough for travel budgets.",
        tools: &[
            ToolData { name: "flutter", category: "frontend", verdict: "love", cost: Some("$0"), why: "Beautiful material design on both platforms. Offline mode with Hive" },
            ToolData { name: "supabase", category: "database", verdict: "love", cost: Some("$25/mo"), why: "Shared trip data with real-time sync. Row-level security per trip" },
            ToolData { name: "cloudflare workers", category: "hosting", verdict: "love", cost: Some("$0"), why: "Edge function for currency conversion API. Cached rates, fast globally" },
            ToolData { name: "firebase", category: "auth", verdict: "good", cost: Some("$0"), why: "Google and Apple sign-in. Anonymous auth for try-before-you-sign-up" },
            ToolData { name: "revenucat", category: "payments", verdict: "love", cost: Some("$0"), why: "Premium tier with receipt validation across both stores" },
        ],
    },
    // 49
    StackSeed {
        project_name: "CodeReviewBot",
        description: "GitHub app that auto-reviews PRs using AI, suggests improvements, and checks for security issues.",
        category: "devtool", scale: "thousands", creator: 30,
        lessons: "LLM context windows are still too small for large PRs. Split diffs into file-level chunks and aggregate the review. Quality is much better.",
        tools: &[
            ToolData { name: "typescript", category: "backend", verdict: "love", cost: Some("$0"), why: "GitHub API types are excellent. Octokit SDK handles auth and pagination" },
            ToolData { name: "openai api", category: "backend", verdict: "good", cost: Some("$300/mo"), why: "GPT-4 for code review is impressive. Claude is better for larger contexts" },
            ToolData { name: "postgresql", category: "database", verdict: "good", cost: Some("$0"), why: "Review history and user preferences. Simple schema" },
            ToolData { name: "vercel", category: "hosting", verdict: "good", cost: Some("$20/mo"), why: "Serverless functions for GitHub webhook handlers. Auto-scaling during PR spikes" },
            ToolData { name: "redis", category: "database", verdict: "good", cost: Some("$0"), why: "Rate limiting per repo to avoid LLM cost explosions" },
        ],
    },
    // 50
    StackSeed {
        project_name: "IndieSaaS Boilerplate",
        description: "Production-ready SaaS starter kit with auth, billing, teams, and admin panel. Ship your MVP in a weekend.",
        category: "devtool", scale: "hundreds", creator: 4,
        lessons: "Don't over-abstract the boilerplate. Users want to understand and modify the code, not fight with your clever abstractions.",
        tools: &[
            ToolData { name: "next.js", category: "frontend", verdict: "love", cost: Some("$0"), why: "Most popular framework means most users can use our boilerplate" },
            ToolData { name: "prisma", category: "database", verdict: "good", cost: Some("$0"), why: "Type-safe database access. Users can switch between Postgres, MySQL, SQLite" },
            ToolData { name: "clerk", category: "auth", verdict: "love", cost: Some("$25/mo"), why: "Drop-in auth with organizations and roles. Saves users weeks" },
            ToolData { name: "stripe", category: "payments", verdict: "love", cost: Some("2.9%+30c"), why: "Billing with usage-based pricing support. Webhooks for subscription lifecycle" },
            ToolData { name: "vercel", category: "hosting", verdict: "love", cost: Some("$20/mo"), why: "One-click deploy button. Users go from clone to production in 5 minutes" },
            ToolData { name: "tailwindcss", category: "frontend", verdict: "love", cost: Some("$0"), why: "Every component is customizable. shadcn/ui integration for beautiful defaults" },
            ToolData { name: "resend", category: "email", verdict: "love", cost: Some("$0"), why: "Generous free tier. React Email templates included in the boilerplate" },
        ],
    },
    // 51
    StackSeed {
        project_name: "StreamDeck Controller",
        description: "Desktop app that turns any tablet into a customizable stream deck with OBS integration.",
        category: "desktop", scale: "hundreds", creator: 26,
        lessons: "Multicast DNS for device discovery works in most home networks but fails in corporate environments. Add manual IP entry as fallback.",
        tools: &[
            ToolData { name: "tauri", category: "frontend", verdict: "love", cost: Some("$0"), why: "Desktop server app is 12MB. Auto-updater built in. Cross-platform without Electron bloat" },
            ToolData { name: "svelte", category: "frontend", verdict: "love", cost: Some("$0"), why: "Grid layout for buttons. Reactive updates when OBS scene changes" },
            ToolData { name: "rust", category: "backend", verdict: "love", cost: Some("$0"), why: "WebSocket server + OBS integration. Low latency for button press to action" },
            ToolData { name: "sqlite", category: "database", verdict: "love", cost: Some("$0"), why: "Button configurations and profiles stored locally. No cloud dependency" },
        ],
    },
    // 52
    StackSeed {
        project_name: "SupplyChain Tracker",
        description: "Inventory and supply chain management for small manufacturers with barcode scanning and PO management.",
        category: "saas", scale: "hundreds", creator: 11,
        lessons: "Barcode scanning in browsers is good enough now. You don't need a native app. The Web Barcode Detection API + camera access works on modern phones.",
        tools: &[
            ToolData { name: "laravel", category: "backend", verdict: "love", cost: Some("$0"), why: "Rapid development with Eloquent. Inventory transactions need proper database patterns" },
            ToolData { name: "vue", category: "frontend", verdict: "love", cost: Some("$0"), why: "Inertia.js with Laravel is the best fullstack DX. No API layer needed" },
            ToolData { name: "mysql", category: "database", verdict: "good", cost: Some("$0"), why: "Works fine with Laravel. Would use Postgres for new projects" },
            ToolData { name: "digitalocean", category: "hosting", verdict: "good", cost: Some("$12/mo"), why: "Managed database + App Platform. Simple and affordable" },
            ToolData { name: "tailwindcss", category: "frontend", verdict: "love", cost: Some("$0"), why: "Dashboard with data tables looks professional with minimal effort" },
        ],
    },
    // 53
    StackSeed {
        project_name: "PodSync",
        description: "Podcast hosting platform with RSS feed generation, analytics, and monetization tools.",
        category: "saas", scale: "tens_of_thousands", creator: 12,
        lessons: "RSS feed compatibility is a nightmare. Apple Podcasts, Spotify, and Google Podcasts all have slightly different requirements. Test on all three.",
        tools: &[
            ToolData { name: "go", category: "backend", verdict: "love", cost: Some("$0"), why: "Audio file processing and RSS generation. Concurrent uploads handled efficiently" },
            ToolData { name: "react", category: "frontend", verdict: "good", cost: Some("$0"), why: "Waveform editor with Web Audio API. Custom audio player component" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "Episode metadata, analytics data, subscriber management. All in one DB" },
            ToolData { name: "cloudflare r2", category: "storage", verdict: "love", cost: Some("$5/mo"), why: "Audio files without egress charges. Our biggest cost saving decision" },
            ToolData { name: "fly.io", category: "hosting", verdict: "love", cost: Some("$15/mo"), why: "Multi-region for fast RSS feed delivery. Podcasters are global" },
            ToolData { name: "stripe", category: "payments", verdict: "love", cost: Some("2.9%+30c"), why: "Creator payouts with Connect. Listener donations with Checkout" },
        ],
    },
    // 54
    StackSeed {
        project_name: "OpenLMS",
        description: "Learning management system for online courses with video hosting, quizzes, and certificates.",
        category: "saas", scale: "thousands", creator: 13,
        lessons: "Video transcoding is expensive and complex. Use Mux or Cloudflare Stream. Don't try to run FFmpeg servers yourself.",
        tools: &[
            ToolData { name: "next.js", category: "frontend", verdict: "love", cost: Some("$0"), why: "SSR for course catalog. Client-side for video player and quiz interactions" },
            ToolData { name: "postgresql", category: "database", verdict: "love", cost: Some("$0"), why: "Course progress tracking, quiz results, enrollment management" },
            ToolData { name: "mux", category: "storage", verdict: "love", cost: Some("$20/mo"), why: "Video hosting with adaptive bitrate streaming. Player SDK is excellent" },
            ToolData { name: "vercel", category: "hosting", verdict: "good", cost: Some("$20/mo"), why: "Fast page loads for course content. Edge functions for auth middleware" },
            ToolData { name: "resend", category: "email", verdict: "good", cost: Some("$20/mo"), why: "Course enrollment confirmations and completion certificates via email" },
            ToolData { name: "stripe", category: "payments", verdict: "love", cost: Some("2.9%+30c"), why: "Course purchases and subscriptions. Coupon codes for promotions" },
        ],
    },
];

const COMMENTS: &[CommentSeed] = &[
    // Stack 0: Orbita POS
    CommentSeed { stack: 0, user: 1, text: "Love the Stripe + Resend combo. We use the same for our billing emails." },
    CommentSeed { stack: 0, user: 3, text: "How do you handle offline POS transactions when the internet drops? That's our biggest challenge." },
    CommentSeed { stack: 0, user: 7, text: "Vercel at $20/mo is a steal for this. We're paying $200 on AWS for a similar setup." },
    CommentSeed { stack: 0, user: 14, text: "Have you considered using Tailwind + shadcn/ui for the admin panel? We switched and it cut our UI dev time in half." },
    // Stack 1: Webhook Relay
    CommentSeed { stack: 1, user: 3, text: "Go is great for this. Have you considered adding OpenTelemetry for distributed tracing?" },
    CommentSeed { stack: 1, user: 20, text: "SQLite for webhooks is bold. What happens when you need horizontal scaling?" },
    CommentSeed { stack: 1, user: 17, text: "We run a similar service. Redis Streams might be a better fit than plain pub/sub for reliable delivery." },
    // Stack 2: FitTrack
    CommentSeed { stack: 2, user: 4, text: "Expo EAS Build is a lifesaver. The old turtle build system was painfully slow." },
    CommentSeed { stack: 2, user: 25, text: "R2 for image storage is genius. We were spending $400/mo on S3 egress before switching." },
    CommentSeed { stack: 2, user: 22, text: "How's Supabase real-time for workout feeds? We had issues with it dropping connections on mobile." },
    // Stack 3: gitkv
    CommentSeed { stack: 3, user: 0, text: "This is such a cool idea. Would love to see a VS Code extension for this." },
    CommentSeed { stack: 3, user: 34, text: "The lesson about tokio is so real. Simple CLIs don't need async. Keep it synchronous." },
    // Stack 4: ShopFlow
    CommentSeed { stack: 4, user: 2, text: "PlanetScale dropping the free tier was rough. We moved to Supabase and haven't looked back." },
    CommentSeed { stack: 4, user: 13, text: "Clerk for auth is expensive but worth it. The pre-built components save weeks." },
    CommentSeed { stack: 4, user: 15, text: "Have you looked at Turso as a PlanetScale replacement? SQLite-based, very cheap." },
    // Stack 5: InvoiceNinja Clone
    CommentSeed { stack: 5, user: 9, text: "Laravel + Vue is the classic combo. Inertia.js makes it even better now." },
    CommentSeed { stack: 5, user: 14, text: "Multi-currency lesson is so true. We spent 3 months getting rounding right across currencies." },
    CommentSeed { stack: 5, user: 0, text: "Why MySQL over Postgres? Curious about the decision." },
    // Stack 6: TeamSync
    CommentSeed { stack: 6, user: 8, text: "Liveblocks at $99/mo is worth every penny. Building real-time collaboration from scratch took us 6 months." },
    CommentSeed { stack: 6, user: 19, text: "How are you handling conflict resolution with the collaborative features?" },
    CommentSeed { stack: 6, user: 12, text: "Server components + Clerk is a great combo. Our auth setup took 30 minutes." },
    CommentSeed { stack: 6, user: 30, text: "Have you benchmarked Prisma vs Drizzle? We switched to Drizzle and queries got 2x faster." },
    // Stack 7: MetricsDash
    CommentSeed { stack: 7, user: 6, text: "shadcn/ui charts are beautiful. Which chart library are you using underneath?" },
    CommentSeed { stack: 7, user: 23, text: "Supabase RLS for multi-tenancy is underrated. So much simpler than application-level checks." },
    // Stack 8: HelpBridge
    CommentSeed { stack: 8, user: 19, text: "MongoDB for chat is a common regret. We started the same way and migrated to Postgres after a year." },
    CommentSeed { stack: 8, user: 6, text: "OpenAI for response suggestions is clever. What's the latency like? Our customers expect instant replies." },
    CommentSeed { stack: 8, user: 31, text: "SSE over WebSocket is great advice. We cut our infrastructure costs by 60% with that switch." },
    // Stack 9: CalSync
    CommentSeed { stack: 9, user: 0, text: "Fastify over Express is the right call. The JSON schema validation alone saves so much boilerplate." },
    CommentSeed { stack: 9, user: 20, text: "Google Calendar API rate limits are criminal. We had to implement exponential backoff with jitter." },
    CommentSeed { stack: 9, user: 12, text: "Fly.io multi-region is perfect for calendar sync. Our users in Asia were getting 2s delays before." },
    // Stack 10: SurveyStack
    CommentSeed { stack: 10, user: 36, text: "Svelte + SvelteKit is the dream stack. Our form builder performance improved 3x after migrating from React." },
    CommentSeed { stack: 10, user: 37, text: "Turso + Drizzle is a combo I want to try. How's the migration story?" },
    CommentSeed { stack: 10, user: 15, text: "Cloudflare Pages free tier is insanely generous. We host 5 production sites on it." },
    // Stack 11: SubTracker
    CommentSeed { stack: 11, user: 4, text: "Dunning email timing insight is gold. We just send at expiry and recover maybe 10%." },
    CommentSeed { stack: 11, user: 9, text: "Postmark deliverability is the best I've tested. Worth the premium over SendGrid." },
    // Stack 12: RecruiterAI
    CommentSeed { stack: 12, user: 24, text: "AI hallucinations in HR context is a real concern. Always have human-in-the-loop." },
    CommentSeed { stack: 12, user: 30, text: "pgvector for semantic search is incredible. We're using it for similar candidate matching." },
    CommentSeed { stack: 12, user: 6, text: "Auth0 enterprise SSO is important for B2B. Clerk's org feature might be cheaper though." },
    // Stack 13: CrateJoy Clone
    CommentSeed { stack: 13, user: 14, text: "Rails + Hotwire is underrated. Turbo Streams give you real-time without JavaScript complexity." },
    CommentSeed { stack: 13, user: 4, text: "Heroku pricing is out of control. Railway gives you more for $5/mo than Heroku for $25." },
    // Stack 14: ArtisanMart
    CommentSeed { stack: 14, user: 9, text: "Django admin for marketplace management is genius. Custom admin actions for bulk operations save hours." },
    CommentSeed { stack: 14, user: 0, text: "Nuxt 3 + Django REST is a solid combo. How do you handle CORS and auth between them?" },
    CommentSeed { stack: 14, user: 22, text: "Full-text search with unaccented indexes — smart thinking for Spanish product names." },
    // Stack 15: DigitalVault
    CommentSeed { stack: 15, user: 4, text: "LemonSqueezy handling VAT is the killer feature. We spent $5k on a tax consultant before finding it." },
    CommentSeed { stack: 15, user: 36, text: "Neon branching for preview environments is something every database should have." },
    CommentSeed { stack: 15, user: 10, text: "ISR for product pages is perfect. Our pages are always fresh but load instantly." },
    // Stack 16: PrintDrop
    CommentSeed { stack: 16, user: 8, text: "Agree about Express. NestJS adds just enough structure without being Spring Boot heavy." },
    CommentSeed { stack: 16, user: 13, text: "MongoDB is actually fine for design data. Not everything needs to be Postgres." },
    // Stack 17: PayGate
    CommentSeed { stack: 17, user: 31, text: "Go for payment processing makes sense. The goroutine model handles concurrent transactions well." },
    CommentSeed { stack: 17, user: 24, text: "SERIALIZABLE isolation for payments — yes. We learned this the hard way with double charges." },
    CommentSeed { stack: 17, user: 6, text: "Datadog APM saved us during our last outage too. The trace waterfall pinpointed the bottleneck in seconds." },
    // Stack 18: ImageKit
    CommentSeed { stack: 18, user: 3, text: "Rust for image processing is perfect. We measured 10x throughput vs our old Node.js sharp setup." },
    CommentSeed { stack: 18, user: 37, text: "Cloudflare Workers + R2 for image transformation is the cheapest setup possible." },
    // Stack 19: NotifyHub
    CommentSeed { stack: 19, user: 8, text: "NestJS dependency injection is great for testing notification providers. Mock the interface, test the logic." },
    CommentSeed { stack: 19, user: 31, text: "BullMQ for notification queues is solid. We process 100k notifications daily with it." },
    CommentSeed { stack: 19, user: 9, text: "Render background workers are underrated. Much simpler than setting up SQS + Lambda." },
    // Stack 20: PDFForge
    CommentSeed { stack: 20, user: 3, text: "Fly Machines API for scaling PDF workers is clever. Pay only when generating PDFs." },
    CommentSeed { stack: 20, user: 34, text: "Playwright over Puppeteer for PDF generation — confirmed. Memory usage is night and day." },
    // Stack 21: BudgetBuddy
    CommentSeed { stack: 21, user: 25, text: "Plaid at $500/mo is brutal for a startup. Have you looked at MX or Yodlee for cheaper alternatives?" },
    CommentSeed { stack: 21, user: 22, text: "Flutter + Firebase is a proven combo for fintech. Firestore security rules need careful auditing though." },
    // Stack 22: MealPrep
    CommentSeed { stack: 22, user: 2, text: "EAS Update for OTA is game-changing. We pushed a critical fix without waiting for App Store review." },
    CommentSeed { stack: 22, user: 21, text: "RevenueCat handles the complexity of cross-platform subscriptions. Receipts validation alone is worth it." },
    CommentSeed { stack: 22, user: 24, text: "USDA nutrition data is free and comprehensive. Smart to self-host instead of paying for an API." },
    // Stack 23: RidePool
    CommentSeed { stack: 23, user: 2, text: "PostGIS is incredible. Finding nearby riders with ST_DWithin is one efficient query." },
    CommentSeed { stack: 23, user: 25, text: "Native apps for maps is the right call. React Native map performance was always our bottleneck." },
    // Stack 24: ZenFlow
    CommentSeed { stack: 24, user: 21, text: "Flutter custom animations for breathing exercises — this is exactly where Flutter shines over RN." },
    CommentSeed { stack: 24, user: 22, text: "R2 for audio files is smart. Egress fees on audio streaming can get insane fast." },
    CommentSeed { stack: 24, user: 2, text: "Amplitude lesson about day 3 churn is valuable. Push notifications at the right time are critical." },
    // Stack 25: LinguaLeap
    CommentSeed { stack: 25, user: 23, text: "Django admin for content management is irreplaceable. We manage 10k vocabulary cards through it." },
    CommentSeed { stack: 25, user: 21, text: "React Native on low-end Android is still a problem. Flutter's Skia renderer handles it better." },
    // Stack 26: CodeSnap
    CommentSeed { stack: 26, user: 3, text: "esbuild for VS Code extensions is a must. The development loop goes from minutes to milliseconds." },
    CommentSeed { stack: 26, user: 34, text: "Reading other extensions' source code is THE tip for VS Code development. The docs are incomplete." },
    // Stack 27: NoteGraph
    CommentSeed { stack: 27, user: 29, text: "Tauri vs Electron — we made the switch and reduced our app size from 180MB to 12MB." },
    CommentSeed { stack: 27, user: 3, text: "SQLite FTS5 for note search is perfect. Results in under 5ms for 50k notes." },
    CommentSeed { stack: 27, user: 38, text: "Prosemirror for rich text is the right choice. We tried Slate and it was full of bugs." },
    // Stack 28: DeployBot
    CommentSeed { stack: 28, user: 3, text: "SSH connection pooling tip is solid. golang.org/x/crypto/ssh has good multiplexing support." },
    CommentSeed { stack: 28, user: 39, text: "Single binary distribution in Go is perfect for CLI tools. No 'install Node/Python first'." },
    // Stack 29: TablePlus Clone
    CommentSeed { stack: 29, user: 27, text: "Tauri + Svelte for a database GUI? This is the stack I want to build with. How's the IPC performance?" },
    CommentSeed { stack: 29, user: 3, text: "8MB vs 200MB — that's the Tauri value proposition right there. Users notice immediately." },
    // Stack 30: TestForge
    CommentSeed { stack: 30, user: 34, text: "AI test generation that validates tests actually fail on wrong behavior — that's the key insight most tools miss." },
    CommentSeed { stack: 30, user: 12, text: "Tree-sitter for multi-language parsing is the right approach. Way more reliable than regex-based parsing." },
    // Stack 31: StatusOwl
    CommentSeed { stack: 31, user: 39, text: "Status page independent from main infra — we learned this the hard way when our AWS region went down." },
    CommentSeed { stack: 31, user: 36, text: "Astro for static status pages is perfect. Zero JS means it loads even on terrible connections." },
    CommentSeed { stack: 31, user: 20, text: "Turso edge database for multi-region health checks is clever. Sub-100ms check intervals." },
    // Stack 32: FlagSmith Clone
    CommentSeed { stack: 32, user: 3, text: "Feature flag expiration dates — wish we had done this. Our flag list is 300+ entries of zombie flags." },
    CommentSeed { stack: 32, user: 31, text: "Redis pub/sub for flag propagation is the right architecture. Instant updates across all instances." },
    // Stack 33: LogRocket Lite
    CommentSeed { stack: 33, user: 31, text: "50MB to 2MB per session — what compression algorithm are you using? We're struggling with similar issues." },
    CommentSeed { stack: 33, user: 6, text: "ClickHouse for event data is perfect. We switched from Postgres and queries went from seconds to milliseconds." },
    // Stack 34: CIForge
    CommentSeed { stack: 34, user: 39, text: "Kaniko over Docker-in-Docker is the right call. Rootless building is also more secure." },
    CommentSeed { stack: 34, user: 28, text: "Vue for build dashboards with real-time logs — we use the same. WebSocket streaming works great." },
    CommentSeed { stack: 34, user: 3, text: "80% cache hit rate cutting build times in half. Redis cache keys based on lockfile hash?" },
    // Stack 35: Blogfolio
    CommentSeed { stack: 35, user: 36, text: "Astro content collections are perfect for blogs. The type safety for frontmatter is a nice bonus." },
    CommentSeed { stack: 35, user: 10, text: "Plausible over Google Analytics is the privacy-respecting choice. No cookie banners needed." },
    // Stack 36: LinkTree Clone
    CommentSeed { stack: 36, user: 37, text: "Drizzle + Turso is such a good combo. Push-based migrations are simpler than Prisma's generate step." },
    CommentSeed { stack: 36, user: 10, text: "Vanity URL handling is deceptively complex. Unicode normalization, reserved words, profanity filtering..." },
    CommentSeed { stack: 36, user: 15, text: "Cloudflare Workers for click analytics is free and fast. Perfect for this use case." },
    // Stack 37: FileDrop
    CommentSeed { stack: 37, user: 38, text: "Hono is my new favorite framework. Runs everywhere. The middleware system is elegant." },
    CommentSeed { stack: 37, user: 20, text: "HTMX for file upload progress without JS frameworks — bold and it works. Love the simplicity." },
    // Stack 38: ChatRoom
    CommentSeed { stack: 38, user: 27, text: "Phoenix Channels for real-time chat is the gold standard. BEAM VM was literally built for telecom." },
    CommentSeed { stack: 38, user: 20, text: "Elixir clustering on Fly.io — how's the node discovery? We had issues with DNS-based clustering." },
    CommentSeed { stack: 38, user: 9, text: "MinIO for self-hosted storage is great for privacy-focused apps. No data leaving your infrastructure." },
    // Stack 39: headlessCMS
    CommentSeed { stack: 39, user: 8, text: "Tiptap for rich text is the right choice. We built our own editor and regret it deeply." },
    CommentSeed { stack: 39, user: 6, text: "JSONB for flexible content schemas is smart. Schema validation at the application layer?" },
    // Stack 40: CartaBien
    CommentSeed { stack: 40, user: 9, text: "Local-first for restaurants is essential. We lost orders during peak hours when WiFi dropped." },
    CommentSeed { stack: 40, user: 14, text: "Supabase real-time for kitchen display is perfect. Orders appear the moment they're placed." },
    CommentSeed { stack: 40, user: 0, text: "PWA capabilities have gotten so good. Service workers cache the full menu offline." },
    // Stack 42: CryptoTracker
    CommentSeed { stack: 42, user: 17, text: "TimescaleDB extension is a game changer for price history. Continuous aggregates for daily candles." },
    CommentSeed { stack: 42, user: 24, text: "Never use floating point for money — this cannot be stressed enough. Use BigNumber everywhere." },
    // Stack 43: FormCraft
    CommentSeed { stack: 43, user: 36, text: "Svelte for drag-and-drop form builders is incredibly smooth. The transition API is beautiful." },
    CommentSeed { stack: 43, user: 10, text: "Uploadthing for file uploads is so much simpler than managing S3 directly." },
    // Stack 44: VetCloud
    CommentSeed { stack: 44, user: 13, text: "Heroku regret is relatable. We migrated to Railway and cut costs by 60%." },
    CommentSeed { stack: 44, user: 9, text: "Twilio for appointment reminders — 40% reduction in no-shows is a huge business impact." },
    // Stack 47: MailForge
    CommentSeed { stack: 47, user: 17, text: "IP warm-up is the most important and most overlooked aspect of email delivery." },
    CommentSeed { stack: 47, user: 19, text: "ClickHouse for email analytics is perfect. We process 10M events daily and queries are instant." },
    // Stack 49: CodeReviewBot
    CommentSeed { stack: 49, user: 34, text: "Splitting diffs into file-level chunks for LLM review is the right approach. Context window limits are real." },
    CommentSeed { stack: 49, user: 12, text: "Claude for larger contexts — we've seen the same. GPT-4 for focused reviews, Claude for big PRs." },
    // Stack 50: IndieSaaS Boilerplate
    CommentSeed { stack: 50, user: 4, text: "Don't over-abstract the boilerplate — this is the best advice. Users need to read and modify the code." },
    CommentSeed { stack: 50, user: 7, text: "Clerk + Stripe + Vercel is the fastest path to SaaS launch. This combo is unbeatable." },
    CommentSeed { stack: 50, user: 13, text: "shadcn/ui in boilerplate makes customization easy. Unlike opinionated component libraries." },
    // Stack 53: PodSync
    CommentSeed { stack: 53, user: 12, text: "R2 for podcast audio is perfect. Egress fees would have killed us with S3." },
    CommentSeed { stack: 53, user: 38, text: "RSS feed compatibility across platforms is a nightmare. Apple is especially picky about artwork specs." },
    // Stack 54: OpenLMS
    CommentSeed { stack: 54, user: 7, text: "Mux for video hosting is worth every penny. Adaptive bitrate streaming without managing FFmpeg." },
    CommentSeed { stack: 54, user: 15, text: "SSR for course catalog + client-side for video player is the right architecture split." },
];

const VOTES: &[VoteSeed] = &[
    // High votes (popular stacks)
    VoteSeed { stack: 0, voters: &[1, 2, 3, 4, 7, 8, 14, 19, 22, 25, 30, 36] },  // Orbita POS: 12
    VoteSeed { stack: 6, voters: &[0, 2, 5, 7, 8, 12, 19, 23, 30, 31] },          // TeamSync: 10
    VoteSeed { stack: 50, voters: &[0, 2, 5, 7, 8, 13, 15, 36, 37] },             // IndieSaaS: 9
    VoteSeed { stack: 15, voters: &[0, 4, 7, 10, 14, 36, 37, 38] },               // DigitalVault: 8
    VoteSeed { stack: 1, voters: &[0, 3, 4, 17, 20, 31, 34] },                    // Webhook Relay: 7
    VoteSeed { stack: 31, voters: &[3, 20, 28, 34, 36, 37, 39] },                 // StatusOwl: 7
    VoteSeed { stack: 17, voters: &[1, 3, 6, 18, 24, 31] },                       // PayGate: 6
    VoteSeed { stack: 38, voters: &[9, 20, 27, 31, 37, 39] },                     // ChatRoom: 6
    VoteSeed { stack: 9, voters: &[0, 8, 12, 19, 20] },                           // CalSync: 5
    VoteSeed { stack: 10, voters: &[15, 36, 37, 38, 39] },                        // SurveyStack: 5
    VoteSeed { stack: 14, voters: &[0, 5, 9, 22, 23] },                           // ArtisanMart: 5
    VoteSeed { stack: 29, voters: &[3, 27, 34, 36, 37] },                         // TablePlus: 5
    VoteSeed { stack: 36, voters: &[10, 15, 37, 38, 39] },                        // LinkTree: 5
    VoteSeed { stack: 18, voters: &[3, 17, 20, 37] },                             // ImageKit: 4
    VoteSeed { stack: 24, voters: &[2, 21, 22, 25] },                             // ZenFlow: 4
    VoteSeed { stack: 37, voters: &[20, 31, 38, 39] },                            // FileDrop: 4
    VoteSeed { stack: 42, voters: &[6, 17, 24, 30] },                             // CryptoTracker: 4
    VoteSeed { stack: 2, voters: &[0, 1, 25] },                                   // FitTrack: 3
    VoteSeed { stack: 3, voters: &[0, 2, 34] },                                   // gitkv: 3
    VoteSeed { stack: 7, voters: &[6, 23, 30] },                                  // MetricsDash: 3
    VoteSeed { stack: 19, voters: &[8, 31, 34] },                                 // NotifyHub: 3
    VoteSeed { stack: 22, voters: &[2, 21, 24] },                                 // MealPrep: 3
    VoteSeed { stack: 26, voters: &[3, 27, 34] },                                 // CodeSnap: 3
    VoteSeed { stack: 27, voters: &[3, 29, 38] },                                 // NoteGraph: 3
    VoteSeed { stack: 32, voters: &[3, 31, 34] },                                 // FlagSmith: 3
    VoteSeed { stack: 43, voters: &[10, 36, 37] },                                // FormCraft: 3
    VoteSeed { stack: 53, voters: &[12, 38, 39] },                                // PodSync: 3
    VoteSeed { stack: 4, voters: &[1, 13] },                                      // ShopFlow: 2
    VoteSeed { stack: 5, voters: &[9, 14] },                                      // InvoiceNinja: 2
    VoteSeed { stack: 8, voters: &[6, 19] },                                      // HelpBridge: 2
    VoteSeed { stack: 11, voters: &[4, 9] },                                      // SubTracker: 2
    VoteSeed { stack: 12, voters: &[24, 30] },                                    // RecruiterAI: 2
    VoteSeed { stack: 13, voters: &[14, 4] },                                     // CrateJoy: 2
    VoteSeed { stack: 16, voters: &[8, 13] },                                     // PrintDrop: 2
    VoteSeed { stack: 20, voters: &[3, 34] },                                     // PDFForge: 2
    VoteSeed { stack: 21, voters: &[22, 25] },                                    // BudgetBuddy: 2
    VoteSeed { stack: 23, voters: &[2, 25] },                                     // RidePool: 2
    VoteSeed { stack: 25, voters: &[23, 21] },                                    // LinguaLeap: 2
    VoteSeed { stack: 28, voters: &[3, 39] },                                     // DeployBot: 2
    VoteSeed { stack: 30, voters: &[12, 34] },                                    // TestForge: 2
    VoteSeed { stack: 33, voters: &[31, 6] },                                     // LogRocket: 2
    VoteSeed { stack: 34, voters: &[28, 39] },                                    // CIForge: 2
    VoteSeed { stack: 35, voters: &[36, 10] },                                    // Blogfolio: 2
    VoteSeed { stack: 39, voters: &[8, 6] },                                      // headlessCMS: 2
    VoteSeed { stack: 40, voters: &[9, 14] },                                     // CartaBien: 2
    VoteSeed { stack: 41, voters: &[8, 13] },                                     // DocuSign: 2
    VoteSeed { stack: 44, voters: &[13, 9] },                                     // VetCloud: 2
    VoteSeed { stack: 45, voters: &[3, 34] },                                     // EdgeCDN: 2
    VoteSeed { stack: 46, voters: &[17, 31] },                                    // APIShield: 2
    VoteSeed { stack: 47, voters: &[17, 19] },                                    // MailForge: 2
    VoteSeed { stack: 48, voters: &[21, 22] },                                    // TravelBudget: 2
    VoteSeed { stack: 49, voters: &[12, 34] },                                    // CodeReviewBot: 2
    VoteSeed { stack: 51, voters: &[27, 29] },                                    // StreamDeck: 2
    VoteSeed { stack: 52, voters: &[11, 14] },                                    // SupplyChain: 2
    VoteSeed { stack: 54, voters: &[7, 15] },                                     // OpenLMS: 2
];

pub async fn seed(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    if std::env::var("RUST_ENV").unwrap_or_default() == "production" {
        return Err(err(StatusCode::FORBIDDEN, "seed is disabled in production"));
    }

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM stacks")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("seed count check: {}", e);
            err(StatusCode::INTERNAL_SERVER_ERROR, "database error")
        })?;

    if count.0 > 0 {
        return Ok(Json(serde_json::json!({
            "message": "already seeded",
            "stacks": count.0
        })));
    }

    // --- Create users ---
    let mut user_ids = Vec::with_capacity(USERS.len());
    for nickname in USERS {
        let id = Uuid::new_v4();
        let recovery_hash = hash_code(&format!("SEED-{}", nickname))
            .map_err(|_| err(StatusCode::INTERNAL_SERVER_ERROR, "hashing error"))?;
        queries::insert_user(&state.pool, id, nickname, &recovery_hash)
            .await
            .map_err(|e| {
                tracing::error!("seed insert_user {}: {}", nickname, e);
                err(StatusCode::INTERNAL_SERVER_ERROR, "could not create user")
            })?;
        user_ids.push(id);
    }
    tracing::info!("seeded {} users", user_ids.len());

    // --- Create stacks with tools ---
    let mut stack_ids = Vec::with_capacity(STACKS.len());
    let mut total_tools = 0usize;

    for stack_data in STACKS {
        let stack_id = Uuid::new_v4();
        let creator_id = user_ids[stack_data.creator];

        queries::insert_stack(
            &state.pool,
            stack_id,
            creator_id,
            stack_data.project_name,
            stack_data.description,
            stack_data.category,
            None,
            stack_data.scale,
            Some(stack_data.lessons),
            None,
        )
        .await
        .map_err(|e| {
            tracing::error!("seed insert_stack {}: {}", stack_data.project_name, e);
            err(StatusCode::INTERNAL_SERVER_ERROR, "could not create stack")
        })?;

        for tool in stack_data.tools {
            queries::insert_stack_tool(
                &state.pool,
                Uuid::new_v4(),
                stack_id,
                tool.name,
                tool.category,
                tool.why,
                tool.cost,
                tool.verdict,
            )
            .await
            .map_err(|e| {
                tracing::error!("seed insert_tool {} / {}: {}", stack_data.project_name, tool.name, e);
                err(StatusCode::INTERNAL_SERVER_ERROR, "could not create tool")
            })?;
            total_tools += 1;
        }

        stack_ids.push(stack_id);
    }
    tracing::info!("seeded {} stacks with {} tools", stack_ids.len(), total_tools);

    // --- Add comments ---
    let mut total_comments = 0usize;
    for c in COMMENTS {
        if c.stack < stack_ids.len() && c.user < user_ids.len() {
            queries::insert_comment(
                &state.pool,
                Uuid::new_v4(),
                stack_ids[c.stack],
                user_ids[c.user],
                c.text,
            )
            .await
            .map_err(|e| {
                tracing::error!("seed insert_comment: {}", e);
                err(StatusCode::INTERNAL_SERVER_ERROR, "could not create comment")
            })?;
            total_comments += 1;
        }
    }
    tracing::info!("seeded {} comments", total_comments);

    // --- Add votes ---
    let mut total_votes = 0usize;
    for v in VOTES {
        if v.stack < stack_ids.len() {
            for &voter in v.voters {
                if voter < user_ids.len() {
                    queries::insert_vote(
                        &state.pool,
                        Uuid::new_v4(),
                        user_ids[voter],
                        stack_ids[v.stack],
                        1,
                    )
                    .await
                    .map_err(|e| {
                        tracing::error!("seed insert_vote: {}", e);
                        err(StatusCode::INTERNAL_SERVER_ERROR, "could not create vote")
                    })?;
                    total_votes += 1;
                }
            }
            queries::recalc_upvotes(&state.pool, stack_ids[v.stack])
                .await
                .map_err(|e| {
                    tracing::error!("seed recalc_upvotes: {}", e);
                    err(StatusCode::INTERNAL_SERVER_ERROR, "could not recalc votes")
                })?;
        }
    }
    tracing::info!("seeded {} votes", total_votes);

    Ok(Json(serde_json::json!({
        "message": "seeded successfully",
        "users": user_ids.len(),
        "stacks": stack_ids.len(),
        "tools": total_tools,
        "comments": total_comments,
        "votes": total_votes
    })))
}
