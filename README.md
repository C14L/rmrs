# Reddmeet Backend in Rust

## Run

Add a PAGESDIR env variable where static files are read from, e.g.

    PAGESDIR="$(pwd)/../frontend" cargo run

## Redis

    session:<session id>
        Hashmap as key-value-store for a given user session, with login
        state, buffered last search results, etc.

    online
        Sorted set of <unixtime> (score) and <user id> (value), to show
        active and idle.

    profile:<user id>
        Hashmap of user profile fields and values.

    votes
        List as a simple queue for up/down votes. To be applied to the
        SQL-DB in bulks.

    blocked
        Set of reddit usernames that are not allowed to register or use
        the site and are never shown on the site.

    sockets
        Set of <user id> that hve video chat enabled and are connected via
        web socket.

    cities
        Hashmap of city names and their lat:lng coordinates to show on the
        site like "/redditors-in-boston.html" and links like "Meet Redditors
        in Boston".

### SQL-DB

    profile
    vote

        User profile data to be used in search by the number of shared
        subreddit subscriptions, sexual preferences, geolocation proximity,
        user's downvote list, etc.
