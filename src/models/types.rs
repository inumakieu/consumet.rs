struct IProviderStats {
    name: String,
    baseUrl: String,
    lang: Vec<String>,
    isNSFW: bool,
    logo: String,
    classPath: String,
    isWorking: bool,
}

struct ITitle {
    romaji: Option<String>,
    english: Option<String>,
    native: Option<String>,
    userPreferred: Option<String>,
}

struct IAnimeResult {
    id: String,
    title: ITitle,
    url: Option<String>,
    image: Option<String>,
    cover: Option<String>,
    status: Option<MediaStatus>,
    rating: Option<u32>,
    showtype: Option<MediaFormat>,
    releaseDate: Option<String>,
}

struct ISearch<T> {
    currentPage: Option<u32>,
    hasNextPage: Option<bool>,
    totalPages: Option<u32>,
    /**
     * total results must include results from all pages
     */
    totalResults: Option<u32>,
    results: Vec<T>,
}

struct Trailer {
    id: String,
    site: Option<String>,
    thumbnail: Option<String>,
}

struct FuzzyDate {
    year: Option<u32>,
    month: Option<u32>,
    day: Option<u32>,
}

enum MediaFormat {
    TV = "TV",
    TV_SHORT = "TV_SHORT",
    MOVIE = "MOVIE",
    SPECIAL = "SPECIAL",
    OVA = "OVA",
    ONA = "ONA",
    MUSIC = "MUSIC",
    MANGA = "MANGA",
    NOVEL = "NOVEL",
    ONE_SHOT = "ONE_SHOT",
}

struct IAnimeInfo {
    malId: Option<u32>,
    genres: Option<Vec<String>>,
    description: Option<String>,
    status: Option<MediaStatus>,
    totalEpisodes: Option<u32>,
    subOrDub: Option<SubOrSub>,
    synonyms: Option<Vec<String>>,
    /**
     * two letter representation of coutnry: e.g JP for japan
     */
    countryOfOrigin: Option<String>,
    isAdult: Option<bool>,
    isLicensed: Option<bool>,
    /**
     * `FALL`, `WINTER`, `SPRING`, `SUMMER`
     */
    season: Option<String>,
    studios: Option<Vec<String>>,
    color: Option<String>,
    cover: Option<String>,
    trailer: Option<Trailer>,
    episodes: Option<Vec<IAnimeEpisode>>,
    startDate: Option<FuzzyDate>,
    endDate: Option<FuzzyDate>,
    recommendations: Option<IAnimeResult>,
    relations: Option<Vec<IAnimeResult>>,
}

struct IAnimeEpisode {
    id: String,
    u32: u32,
    title: Option<String>,
    description: Option<String>,
    isFiller: Option<bool>,
    url: Option<String>,
    image: Option<String>,
    releaseDate: Option<String>,
}

struct IEpisodeServer {
    name: String,
    url: String,
}

struct IVideo {
    /**
     * The **MAIN URL** of the video provider that should take you to the video
     */
    url: String,
    /**
     * The Quality of the video should include the `p` suffix
     */
    quality: Option<String>,
    /**
     * make sure to set this to `true` if the video is hls
     */
    isM3U8: Option<bool>,
    /**
     * size of the video in **bytes**
     */
    size: Option<u32>,
}

enum StreamingServers {
    GogoCDN = "gogocdn",
    StreamSB = "streamsb",
    MixDrop = "mixdrop",
    UpCloud = "upcloud",
    VidCloud = "vidcloud",
    StreamTape = "streamtape",
    VizCloud = "vizcloud",
    // same as vizcloud
    MyCloud = "mycloud",
    Filemoon = "filemoon",
    VidStreaming = "vidstreaming",
}

enum MediaStatus {
    ONGOING = "Ongoing",
    COMPLETED = "Completed",
    HIATUS = "Hiatus",
    CANCELLED = "Cancelled",
    NOT_YET_AIRED = "Not yet aired",
    UNKNOWN = "Unknown",
}

enum SubOrSub {
    SUB = "sub",
    DUB = "dub",
    BOTH = "both",
}

struct IMangaResult {
    id: String,
    title: String,
    altTitles: Option<Vec<String>>,
    image: Option<String>,
    description: Option<String>,
    status: Option<MediaStatus>,
    releaseDate: Option<u32>,
}

struct IMangaChapter {
    id: String,
    title: String,
    volume: Option<u32>,
    pages: Option<u32>,
    releaseDate: Option<String>,
}

struct IMangaInfo {
    malId: Option<u32>,
    authors: Option<Vec<String>>,
    genres: Option<Vec<String>>,
    links: Option<Vec<String>>,
    characters: Option<Vec<any>>,
    recommendations: Option<Vec<IMangaResult>>,
    chapters: Option<Vec<IMangaChapter>>,
}

struct IMangaChapterPage {
    img: String,
    page: u32,
}

struct ILightNovelResult {
    id: String,
    title: String,
    url: String,
    image: Option<String>,
}

struct ILightNovelChapter {
    id: String,
    title: String,
    volume: Option<u32>,
    url: Option<String>,
}

struct ILightNovelChapterContent {
    text: String,
    html: Option<String>,
}

struct ILightNovelInfo {
    authors: Option<Vec<String>>,
    genres: Option<Vec<String>>,
    description: Option<String>,
    chapters: Option<Vec<ILightNovelChapter>>,
    status: Option<MediaStatus>,
    views: Option<u32>,
    rating: Option<u32>,
}

struct LibgenBook {
    id: String,
    language: String,
    format: String,
    size: String,
    pages: String,
    tableOfContents: String,
    topic: String,
    hashes: Hashes,
}

struct LibgenResult {
    result: Vec<LibgenBook>,
    hasNextPage: bool,
}

struct GetComicsComics {
    image: String,
    title: String,
    year: String,
    size: String,
    excerpt: String,
    category: String,
    description: String,
    download: String,
    ufile: String,
    mega: String,
    mediafire: String,
    zippyshare: String,
    readOnline: String,
}

struct ComicRes {
    containers: Vec<GetComicsComics>,
    hasNextPage: bool,
}

struct ZLibrary {
    bookRating: String,
    bookQuality: String,
    language: String,
    size: String,
    pages: String,
}

struct ISubtitle {
    /**
     * The **url** that should take you to the subtitle **directly**.
     */
    url: String,
    /**
     * The language of the subtitle
     */
    lang: String,
}

/**
 * The start, and the end of the intro or opening in seconds.
 */
struct Intro {
    start: u32,
    end: u32,
}

struct ISource {
    headers: Option<String>,
    intro: Option<Intro>,
    subtitles: Option<Vec<ISubtitle>>,
    sources: Vec<IVideo>,
}

/**
 * Used **only** for movie/tvshow providers
 */
enum TvType {
    TVSERIES = "TV Series",
    MOVIE = "Movie",
    ANIME = "Anime",
}

struct IMovieEpisode {
    id: String,
    title: String,
    url: Option<String>,
    u32: Option<u32>,
    season: Option<u32>,
    image: Option<String>,
    releaseDate: Option<String>,
}

struct IMovieResult {
    id: String,
    title: String,
    url: Option<String>,
    image: Option<String>,
    releaseDate: Option<String>,
    movietype: Option<TvType>,
}

struct INewsFeed {
    /** topics of the feed */
    topics: Vec<Topics>,
    /** preview of the news feed */
    preview: INewsFeedPreview,
}

struct INewsInfo {
    /** intro of the news */
    intro: String,
    /** description of the news */
    description: String,
}

struct INews {
    /** id of the news */
    id: String,
    /** title of the news */
    title: String,
    /** time at which the news was uploaded */
    uploadedAt: String,
    /** thumbnail image URL of the news */
    thumbnail: String,
    /** URL of the news */
    url: String,
}

struct INewsFeedPreview {
    /** intro of the feed */
    intro: String,
    /** some contents of the feed */
    full: String,
}

struct IMovieInfo {
    genres: Option<Vec<String>>,
    description: Option<String>,
    rating: Option<u32>,
    status: Option<MediaStatus>,
    duration: Option<String>,
    production: Option<String>,
    casts: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    totalEpisodes: Option<u32>,
    episodes: Option<Vec<IMovieEpisode>>,
}

enum Genres {
    ACTION = "Action",
    ADVENTURE = "Adventure",
    CARS = "Cars",
    COMEDY = "Comedy",
    DRAMA = "Drama",
    ECCHI = "Ecchi",
    FANTASY = "Fantasy",
    HORROR = "Horror",
    MAHOU_SHOUJO = "Mahou Shoujo",
    MECHA = "Mecha",
    MUSIC = "Music",
    MYSTERY = "Mystery",
    PSYCHOLOGICAL = "Psychological",
    ROMANCE = "Romance",
    SCI_FI = "Sci-Fi",
    SLICE_OF_LIFE = "Slice of Life",
    SPORTS = "Sports",
    SUPERNATURAL = "Supernatural",
    THRILLER = "Thriller",
}

enum Topics {
    ANIME = "anime",
    ANIMATION = "animation",
    MANGA = "manga",
    GAMES = "games",
    NOVELS = "novels",
    LIVE_ACTION = "live-action",
    COVID_19 = "covid-19",
    INDUSTRY = "industry",
    MUSIC = "music",
    PEOPLE = "people",
    MERCH = "merch",
    EVENTS = "events",
}

struct ProxyConfig {
    /**
     * The proxy URL
     * @example https://proxy.com
     **/
    url: String,
    /**
     * X-API-Key header value (if any)
     **/
    key: Option<String>,
}
