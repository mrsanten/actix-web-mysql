CREATE DATABASE IF NOT EXISTS `actix_web_mysql`;
USE `actix_web_mysql`;

DROP TABLE IF EXISTS `users_to_groups`;
DROP TABLE IF EXISTS `posts`;
DROP TABLE IF EXISTS `groups`;
DROP TABLE IF EXISTS `users`;

CREATE TABLE IF NOT EXISTS users 
(
	id VARCHAR(48) NOT NULL UNIQUE,
	name VARCHAR(64) NOT NULL UNIQUE,
	email VARCHAR(256) NOT NULL UNIQUE,
	PRIMARY KEY (id)
);
            
CREATE TABLE IF NOT EXISTS `groups`
(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(64) NOT NULL UNIQUE,
    PRIMARY KEY(id)
);
            
CREATE TABLE IF NOT EXISTS `users_to_groups`
(
    `user_id` VARCHAR(48) NOT NULL,
    `group_id` BIGINT UNSIGNED NOT NULL,
    FOREIGN KEY (`user_id`) REFERENCES `users`(`id`),
    FOREIGN KEY (`group_id`) REFERENCES `groups`(`id`)
);

CREATE TABLE IF NOT EXISTS `posts`
(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `user_id` VARCHAR(48) NOT NULL,
    `title` VARCHAR(128) NOT NULL,
    `content` TEXT NOT NULL,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY(id),
    FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
);

CREATE USER IF NOT EXISTS 'actix_web_mysql_user'@'localhost' IDENTIFIED BY 'actix_web_mysql_password';
GRANT SELECT, INSERT, UPDATE, DELETE ON `actix_web_mysql`.* TO 'actix_web_mysql_user'@'localhost';