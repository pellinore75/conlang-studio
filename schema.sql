-- schema.sql
-- 1. USERS
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 2. PROJECTS
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL, -- New owner field
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 3. LANGUAGES (The "Nodes")
CREATE TABLE IF NOT EXISTS languages (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,         -- e.g., "Proto-Lang"
    type TEXT NOT NULL,         -- "proto", "evolved", "constructed"
    parent_language_id INTEGER, -- Links "Modern" back to "Proto"
    FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY(parent_language_id) REFERENCES languages(id)
);

-- 4. PHONOLOGIES (The "DNA")
CREATE TABLE IF NOT EXISTS phonologies (
    language_id INTEGER PRIMARY KEY,
    data JSON NOT NULL, -- { "vowels": ["a", "i", "u"], "stress": "penultimate" }
    FOREIGN KEY(language_id) REFERENCES languages(id) ON DELETE CASCADE
);

-- 5. EVOLUTION RULES (The "Edges")
CREATE TABLE IF NOT EXISTS evolution_rules (
    id INTEGER PRIMARY KEY,
    parent_lang_id INTEGER NOT NULL,
    child_lang_id INTEGER NOT NULL,
    rule_order INTEGER NOT NULL,
    regex_match TEXT NOT NULL,
    regex_replace TEXT NOT NULL,
    description TEXT,
    FOREIGN KEY(parent_lang_id) REFERENCES languages(id) ON DELETE CASCADE,
    FOREIGN KEY(child_lang_id) REFERENCES languages(id) ON DELETE CASCADE
);

-- 6. LEXICON (The Words)
CREATE TABLE IF NOT EXISTS words (
    id INTEGER PRIMARY KEY,
    language_id INTEGER NOT NULL,
    word TEXT NOT NULL,
    ipa TEXT,
    gloss TEXT NOT NULL,
    definition TEXT,
    part_of_speech TEXT,
    source_word_id INTEGER, -- Link to parent word
    FOREIGN KEY(language_id) REFERENCES languages(id) ON DELETE CASCADE,
    FOREIGN KEY(source_word_id) REFERENCES words(id)
);
