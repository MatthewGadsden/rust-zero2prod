CREATE TABLE subscriptions(
        id uuid NOT NULL,
        PRIMARY KEY (id),
        email TEXT NOT NULL,
        name TEXT NOT NULL,
        subscribed_at timestamptz NOT NULL
);

ALTER TABLE subscriptions ADD CONSTRAINT ux_subscriptions_email UNIQUE (email);
