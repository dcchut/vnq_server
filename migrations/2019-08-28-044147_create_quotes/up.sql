CREATE TABLE `quotes` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    `content` TEXT NOT NULL,
    `votes` INTEGER NOT NULL,
    `visible` TINYINT(1) NOT NULL,
    `moderated_by` INTEGER,
    `ip` VARCHAR(255) NOT NULL,
    `created_at` DATETIME NOT NULL,
    `updated_at` DATETIME NOT NULL,
    `user_id` INTEGER REFERENCES `users` (`id`) ON DELETE SET NULL ON UPDATE CASCADE
)