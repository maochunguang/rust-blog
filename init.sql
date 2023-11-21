CREATE TABLE blog_users (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '用户唯一标识',
    username VARCHAR(50) NOT NULL COMMENT '用户名',
    password_hash VARCHAR(255) NOT NULL COMMENT '存储加密后的密码',
    email VARCHAR(100) COMMENT '用户电子邮件地址',
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '记录创建时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='存储用户信息';
CREATE TABLE blog_posts (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '文章唯一标识',
    user_id BIGINT NOT NULL COMMENT '发布文章的用户ID',
    title VARCHAR(100) NOT NULL COMMENT '文章标题',
    content VARCHAR(2048) NOT NULL COMMENT '文章内容',
    published BOOLEAN DEFAULT FALSE COMMENT '文章是否已发布',
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '文章创建时间',
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '文章最后更新时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='存储博客文章的详细信息';
CREATE TABLE blog_comments (
    id BIGINT AUTO_INCREMENT PRIMARY KEY COMMENT '评论唯一标识',
    post_id BIGINT NOT NULL COMMENT '评论所属的文章ID',
    user_id BIGINT NOT NULL COMMENT '发表评论的用户ID',
    content TEXT NOT NULL COMMENT '评论内容',
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '评论创建时间',
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '评论最后更新时间'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='存储用户对文章的评论';
