export type Locale = "en" | "es";

export const locales: Locale[] = ["en", "es"];

export const defaultLocale: Locale = "en";

export const dictionaries: Record<Locale, Record<string, string>> = {
  en: {
    // nav
    "nav.browse": "browse",
    "nav.tools": "tools",
    "nav.compare": "compare",
    "nav.share": "share your stack",
    "nav.join": "join",
    "nav.logout": "logout",
    "nav.profile": "my profile",

    // nav.account
    "nav.account.anonymous": "anonymous account",
    "nav.account.showCode": "show recovery code",
    "nav.account.hideCode": "hide recovery code",
    "nav.account.codeNotSaved":
      "code not saved in this browser. logout and recover with your code to save it here.",

    // common
    "common.copy": "copy",
    "common.copied": "copied",
    "common.loading": "loading",
    "common.search": "search",
    "common.cancel": "cancel",
    "common.remove": "remove",
    "common.back": "← back to stacks",
    "common.stackNotFound": "stack not found",
    "common.userNotFound": "user not found",

    // footer
    "footer.builtWith": "built with rust + next.js",
    "footer.tagline": "anonymous. honest. real.",

    // home
    "home.subtitle":
      "real stacks from real projects. no theory. no tutorials.",
    "home.stacks": "stacks",
    "home.tools": "tools",
    "home.developers": "developers",
    "home.searchPlaceholder": "search stacks...",
    "home.searchBtn": "search",
    "home.exampleStack": "example stack",
    "home.featuredStack": "featured stack",
    "home.trendingTools": "trending tools",
    "home.trendingWeek": "trending this week",
    "home.hotStacks": "hot stacks",
    "home.hotTools": "hot tools",
    "home.mostRegretted": "most regretted",
    "home.latestStacks": "latest stacks",
    "home.viewAll": "view all →",
    "home.noStacks": "no stacks shared yet. be the first.",
    "home.shareStack": "share your stack →",
    "home.by": "by",

    // stacks
    "stacks.title": "browse stacks",
    "stacks.new": "new",
    "stacks.top": "top",
    "stacks.allCategories": "all categories",
    "stacks.allScales": "all scales",
    "stacks.searchPlaceholder": "search stacks...",
    "stacks.filterTool": "filter by tool...",
    "stacks.noResults": "nothing found. try different filters.",
    "stacks.noStacks": "no stacks shared yet. be the first.",
    "stacks.shareStack": "share your stack →",
    "stacks.loadMore": "load more",

    // stack
    "stack.copyLink": "copy link",
    "stack.editStack": "edit stack",
    "stack.delete": "delete",
    "stack.deleteConfirm": "are you sure? this cannot be undone.",
    "stack.yesDelete": "yes, delete",
    "stack.tools": "tools",
    "stack.lessonsLearned": "lessons learned",
    "stack.exploreTools": "explore these tools",
    "stack.whoUses": "who else uses",
    "stack.comments": "comments",
    "stack.writePlaceholder": "write a comment...",
    "stack.pickNickname": "pick a nickname to comment",
    "stack.post": "post",
    "stack.noComments": "no comments yet. share your thoughts.",
    "stack.changeHistory": "change history",
    "stack.updated": "updated",
    "stack.forkedFrom": "forked from",
    "stack.forkStack": "fork this stack",
    "stack.bookmark": "bookmark",
    "stack.bookmarked": "bookmarked",
    "stack.exportImage": "export as image",
    "stack.exporting": "exporting...",

    // vote
    "vote.already": "you already voted",
    "vote.upvoted": "upvoted!",
    "vote.downvoted": "downvoted",
    "vote.removed": "vote removed",
    "vote.error": "vote failed. try again.",

    // toast
    "toast.bookmarkAdded": "bookmarked!",
    "toast.bookmarkRemoved": "bookmark removed",
    "toast.bookmarkError": "couldn't bookmark. try again.",
    "toast.commentPosted": "comment posted",

    // bookmarks
    "nav.bookmarks": "bookmarks",
    "bookmarks.title": "your bookmarks",
    "bookmarks.empty":
      "no bookmarks yet. browse stacks and save the ones you like.",
    "bookmarks.joinFirst": "join to save stacks",

    // tools.dir
    "tools.dir.title": "tools directory",
    "tools.dir.subtitle":
      "every tool used across all stacks, grouped by category",
    "tools.dir.filterPlaceholder": "filter tools...",
    "tools.dir.noMatch": "no tools found matching",
    "tools.dir.noTools": "no tools found. stacks need to be shared first.",
    "tools.dir.stacks": "stack|stacks",

    // tool
    "tool.usedIn": "use this",
    "tool.whatDevsSay": "what developers say",
    "tool.noUsage": "no one has used this tool yet.",
    "tool.beFirst": "be the first to share",
    "tool.pairedWith": "commonly paired with",
    "tool.alternatives": "developers who regretted this switched to",
    "tool.timesChosen": "chosen",

    // compare
    "compare.title": "compare tools",
    "compare.subtitle":
      "side-by-side comparison based on real developer stacks",
    "compare.tool1": "tool 1",
    "compare.tool2": "tool 2",
    "compare.selectTool": "select a tool...",
    "compare.vs": "vs",
    "compare.compare": "compare",
    "compare.selectTwo": "select two different tools",
    "compare.usedTogether": "used together in",
    "compare.whatDevsSay": "what devs say",
    "compare.pricing": "pricing",

    // new
    "new.title": "share your stack",
    "new.subtitle":
      "what are you building? what tools do you use? be honest.",
    "new.projectName": "project name",
    "new.description": "description",
    "new.category": "category",
    "new.scale": "scale",
    "new.projectUrl": "project url",
    "new.optional": "optional",
    "new.tools": "tools",
    "new.toolN": "tool",
    "new.addTool": "+ add tool",
    "new.lessonsLabel": "lessons learned",
    "new.lessonsPlaceholder": "what went wrong? what would you change?",
    "new.publish": "publish stack",
    "new.publishing": "publishing...",

    // edit
    "edit.title": "edit stack",
    "edit.cancel": "cancel",
    "edit.save": "save changes",
    "edit.saving": "saving...",
    "edit.notFound": "stack not found",
    "edit.backStacks": "back to stacks",
    "edit.notOwner": "you can only edit your own stacks",
    "edit.backStack": "back to stack",
    "edit.toolName": "tool name",
    "edit.whyTool": "why this tool?",
    "edit.cost": "cost (e.g. $20/mo, free)",

    // auth
    "auth.nicknameTaken":
      "that nickname is already taken. try another.",
    "auth.joinTitle": "join stackpedia",
    "auth.recoverTitle": "recover account",
    "auth.newHere": "new here",
    "auth.haveCode": "i have a code",
    "auth.joinDesc":
      "no email. no password. no tracking. pick a name and you're in.",
    "auth.nickPlaceholder": "your nickname",
    "auth.goAnonymous": "go anonymous",
    "auth.codeHint":
      "you'll get a recovery code to save. that's your only key.",
    "auth.recoverDesc":
      "paste your recovery code to get back into your account.",
    "auth.codePlaceholder": "paste your recovery code",
    "auth.recover": "recover",
    "auth.welcomeTitle": "you're in",
    "auth.welcomeDesc":
      "this is your recovery code. it's the only way to get back into your account. save it now.",
    "auth.noRecovery":
      "no email. no password. just this code. we can't recover it for you.",
    "auth.savedIt": "i saved it — let's go",

    // profile
    "profile.joined": "joined",
    "profile.stacks": "stacks",
    "profile.noStacks": "no stacks shared yet.",
    "profile.sponsor": "sponsor",
    "profile.sponsorEdit": "sponsor link",
    "profile.sponsorPlaceholder": "https://github.com/sponsors/you",
    "profile.sponsorSave": "save",
    "profile.sponsorSaved": "saved!",
    "profile.sponsorHint": "github sponsors, buy me a coffee, ko-fi, etc.",

    // cost
    "cost.breakdown": "cost breakdown",
    "cost.free": "free",
    "cost.paid": "paid",
    "cost.variable": "variable",
    "cost.unlisted": "unlisted",

    // meta.home
    "meta.home.title": "Stackpedia — real stacks from real projects",
    "meta.home.desc":
      "Developers share the real tech stack of their production projects. What tools they use, why, what they pay, what failed them. Anonymous, honest, real.",
    "meta.home.ogDesc":
      "Developers share the real tech stack of their production projects. No tutorials, no theory — just real stacks.",

    // meta.compare
    "meta.compare.title": "Compare Tools — Stackpedia",
    "meta.compare.desc":
      "Side-by-side comparison of developer tools based on real-world usage in tech stacks.",
  },

  es: {
    // nav
    "nav.browse": "explorar",
    "nav.tools": "herramientas",
    "nav.compare": "comparar",
    "nav.share": "comparte tu stack",
    "nav.join": "entrar",
    "nav.logout": "salir",
    "nav.profile": "mi perfil",

    // nav.account
    "nav.account.anonymous": "cuenta anónima",
    "nav.account.showCode": "mostrar código de recuperación",
    "nav.account.hideCode": "ocultar código de recuperación",
    "nav.account.codeNotSaved":
      "código no guardado en este navegador. cierra sesión y recupéralo con tu código para guardarlo aquí.",

    // common
    "common.copy": "copiar",
    "common.copied": "copiado",
    "common.loading": "cargando",
    "common.search": "buscar",
    "common.cancel": "cancelar",
    "common.remove": "eliminar",
    "common.back": "← volver a stacks",
    "common.stackNotFound": "stack no encontrado",
    "common.userNotFound": "usuario no encontrado",

    // footer
    "footer.builtWith": "hecho con rust + next.js",
    "footer.tagline": "anónimo. honesto. real.",

    // home
    "home.subtitle":
      "stacks reales de proyectos reales. sin teoría. sin tutoriales.",
    "home.stacks": "stacks",
    "home.tools": "herramientas",
    "home.developers": "desarrolladores",
    "home.searchPlaceholder": "buscar stacks...",
    "home.searchBtn": "buscar",
    "home.exampleStack": "stack de ejemplo",
    "home.featuredStack": "stack destacado",
    "home.trendingTools": "herramientas en tendencia",
    "home.trendingWeek": "tendencia esta semana",
    "home.hotStacks": "stacks populares",
    "home.hotTools": "herramientas populares",
    "home.mostRegretted": "más lamentadas",
    "home.latestStacks": "últimos stacks",
    "home.viewAll": "ver todos →",
    "home.noStacks": "aún no hay stacks. sé el primero.",
    "home.shareStack": "comparte tu stack →",
    "home.by": "por",

    // stacks
    "stacks.title": "explorar stacks",
    "stacks.new": "nuevo",
    "stacks.top": "top",
    "stacks.allCategories": "todas las categorías",
    "stacks.allScales": "todas las escalas",
    "stacks.searchPlaceholder": "buscar stacks...",
    "stacks.filterTool": "filtrar por herramienta...",
    "stacks.noResults": "sin resultados. prueba otros filtros.",
    "stacks.noStacks": "aún no hay stacks. sé el primero.",
    "stacks.shareStack": "comparte tu stack →",
    "stacks.loadMore": "cargar más",

    // stack
    "stack.copyLink": "copiar enlace",
    "stack.editStack": "editar stack",
    "stack.delete": "eliminar",
    "stack.deleteConfirm": "¿estás seguro? esto no se puede deshacer.",
    "stack.yesDelete": "sí, eliminar",
    "stack.tools": "herramientas",
    "stack.lessonsLearned": "lecciones aprendidas",
    "stack.exploreTools": "explorar estas herramientas",
    "stack.whoUses": "quién más usa",
    "stack.comments": "comentarios",
    "stack.writePlaceholder": "escribe un comentario...",
    "stack.pickNickname": "elige un apodo para comentar",
    "stack.post": "publicar",
    "stack.noComments": "sin comentarios aún. comparte tu opinión.",
    "stack.changeHistory": "historial de cambios",
    "stack.updated": "actualizado",
    "stack.forkedFrom": "bifurcado de",
    "stack.forkStack": "bifurcar este stack",
    "stack.bookmark": "guardar",
    "stack.bookmarked": "guardado",
    "stack.exportImage": "exportar como imagen",
    "stack.exporting": "exportando...",

    // vote
    "vote.already": "ya votaste",
    "vote.upvoted": "¡voto a favor!",
    "vote.downvoted": "voto en contra",
    "vote.removed": "voto eliminado",
    "vote.error": "error al votar. intenta de nuevo.",

    // toast
    "toast.bookmarkAdded": "¡guardado!",
    "toast.bookmarkRemoved": "guardado eliminado",
    "toast.bookmarkError": "no se pudo guardar. intenta de nuevo.",
    "toast.commentPosted": "comentario publicado",

    // bookmarks
    "nav.bookmarks": "guardados",
    "bookmarks.title": "tus guardados",
    "bookmarks.empty":
      "sin guardados aún. explora stacks y guarda los que te gusten.",
    "bookmarks.joinFirst": "únete para guardar stacks",

    // tools.dir
    "tools.dir.title": "directorio de herramientas",
    "tools.dir.subtitle":
      "todas las herramientas usadas en los stacks, agrupadas por categoría",
    "tools.dir.filterPlaceholder": "filtrar herramientas...",
    "tools.dir.noMatch": "no se encontraron herramientas con",
    "tools.dir.noTools":
      "no se encontraron herramientas. primero hay que compartir stacks.",
    "tools.dir.stacks": "stack|stacks",

    // tool
    "tool.usedIn": "usan esto",
    "tool.whatDevsSay": "qué dicen los desarrolladores",
    "tool.noUsage": "nadie ha usado esta herramienta aún.",
    "tool.beFirst": "sé el primero en compartir",
    "tool.pairedWith": "comúnmente combinada con",
    "tool.alternatives": "desarrolladores que lamentaron esta cambiaron a",
    "tool.timesChosen": "elegida",

    // compare
    "compare.title": "comparar herramientas",
    "compare.subtitle":
      "comparación lado a lado basada en stacks reales de desarrolladores",
    "compare.tool1": "herramienta 1",
    "compare.tool2": "herramienta 2",
    "compare.selectTool": "selecciona una herramienta...",
    "compare.vs": "vs",
    "compare.compare": "comparar",
    "compare.selectTwo": "selecciona dos herramientas diferentes",
    "compare.usedTogether": "usadas juntas en",
    "compare.whatDevsSay": "qué dicen los devs",
    "compare.pricing": "precios",

    // new
    "new.title": "comparte tu stack",
    "new.subtitle":
      "¿qué estás construyendo? ¿qué herramientas usas? sé honesto.",
    "new.projectName": "nombre del proyecto",
    "new.description": "descripción",
    "new.category": "categoría",
    "new.scale": "escala",
    "new.projectUrl": "url del proyecto",
    "new.optional": "opcional",
    "new.tools": "herramientas",
    "new.toolN": "herramienta",
    "new.addTool": "+ agregar herramienta",
    "new.lessonsLabel": "lecciones aprendidas",
    "new.lessonsPlaceholder": "¿qué salió mal? ¿qué cambiarías?",
    "new.publish": "publicar stack",
    "new.publishing": "publicando...",

    // edit
    "edit.title": "editar stack",
    "edit.cancel": "cancelar",
    "edit.save": "guardar cambios",
    "edit.saving": "guardando...",
    "edit.notFound": "stack no encontrado",
    "edit.backStacks": "volver a stacks",
    "edit.notOwner": "solo puedes editar tus propios stacks",
    "edit.backStack": "volver al stack",
    "edit.toolName": "nombre de la herramienta",
    "edit.whyTool": "¿por qué esta herramienta?",
    "edit.cost": "costo (ej. $20/mes, gratis)",

    // auth
    "auth.nicknameTaken":
      "ese apodo ya está en uso. prueba otro.",
    "auth.joinTitle": "únete a stackpedia",
    "auth.recoverTitle": "recuperar cuenta",
    "auth.newHere": "soy nuevo",
    "auth.haveCode": "tengo un código",
    "auth.joinDesc":
      "sin email. sin contraseña. sin rastreo. elige un nombre y listo.",
    "auth.nickPlaceholder": "tu apodo",
    "auth.goAnonymous": "entrar anónimo",
    "auth.codeHint":
      "recibirás un código de recuperación para guardar. es tu única llave.",
    "auth.recoverDesc":
      "pega tu código de recuperación para volver a tu cuenta.",
    "auth.codePlaceholder": "pega tu código de recuperación",
    "auth.recover": "recuperar",
    "auth.welcomeTitle": "ya estás dentro",
    "auth.welcomeDesc":
      "este es tu código de recuperación. es la única forma de volver a tu cuenta. guárdalo ahora.",
    "auth.noRecovery":
      "sin email. sin contraseña. solo este código. no podemos recuperarlo por ti.",
    "auth.savedIt": "lo guardé — vamos",

    // profile
    "profile.joined": "se unió",
    "profile.stacks": "stacks",
    "profile.noStacks": "aún no ha compartido stacks.",
    "profile.sponsor": "patrocinar",
    "profile.sponsorEdit": "enlace de patrocinio",
    "profile.sponsorPlaceholder": "https://github.com/sponsors/tu",
    "profile.sponsorSave": "guardar",
    "profile.sponsorSaved": "¡guardado!",
    "profile.sponsorHint": "github sponsors, buy me a coffee, ko-fi, etc.",

    // cost
    "cost.breakdown": "desglose de costos",
    "cost.free": "gratis",
    "cost.paid": "de pago",
    "cost.variable": "variable",
    "cost.unlisted": "no listado",

    // meta.home
    "meta.home.title": "Stackpedia — stacks reales de proyectos reales",
    "meta.home.desc":
      "Desarrolladores comparten el stack real de sus proyectos en producción. Qué herramientas usan, por qué, cuánto pagan, qué les falló. Anónimo, honesto, real.",
    "meta.home.ogDesc":
      "Desarrolladores comparten el stack real de sus proyectos en producción. Sin tutoriales, sin teoría — solo stacks reales.",

    // meta.compare
    "meta.compare.title": "Comparar Herramientas — Stackpedia",
    "meta.compare.desc":
      "Comparación lado a lado de herramientas de desarrollo basada en uso real en stacks tecnológicos.",
  },
};

export function getT(locale: Locale): (key: string) => string {
  const dict = dictionaries[locale] ?? dictionaries[defaultLocale];
  return (key: string): string => dict[key] ?? key;
}
