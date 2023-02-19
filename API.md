# Endpoint

`https://moehreag.duckdns.org/axolotlclient-api/<version>/<route>`

Current API version: v1

# Routes

- GET `/count`

    Get the count of known and currently online users.

    Sample response:

    - Success:

        ```json
        {
            "online": 1,
            "total": 2
        }
        ```
        Status code: 200

    - Failure:

        ```json
        {
            "Message": "Not Found"
        }
        ```
        Status code: 404

        Description: The provided UUID is not known to this service.


- GET `/user/<uuid>`

    Get known information about the user with the provided uuid.

    Sample response:

    - Sucess:

        ```json
        {
            "friends": [],
            "online": false,
            "uuid": "6392bacb8b00436682b33abd134e83ac"
        }
        ```
        Status code: 200

    - Failure:

        ```json
        {
            "message": "Not Found"
        }
        ```
        Status code: 404

        Description: The provided UUID is not known to this service.


- POST `/user/<uuid>`

    Create/Update the status of a user.

    Sample request body:

    ```json
    {
        "online": true
    }
    ```

    Sample response:

    - Success:

        ```
        OK
        ```
        Status code: 200

    - Failure:

        ```json
        {
            "error_message": "UUID 6392bacb8b00436682b33aaa134e83ac does not appear to be valid!"
        }
        ```
        Status code: 401

        Description: The provided UUID does not have a valid Minecraft account attached to it.

- GET `/user/<uuid>/friends`

    Get a user's friends.

    Sample response:

    - Success:

        ```json
        [
            {
                "uuid": "6392bacb8b00436682b33aaa134e83ac1",
                "online": false
            }
        ]
        ```
        Status code: 200

    - Failure:

        ```json
        {
            "Message": "Not Found"
        }
        ```
        Status code: 404

        Description: The provided UUID is not known to this service.

- POST `/user/<uuid>/friends`

    Add a friend to a user.

    Sample request body:

    ```json
    {
        "friend": "6392bacb8b00436682b33aaa134e83ac1"
    }
    ```

    Sample response:

    - Success:

        ```
        OK
        ```
        Status code: 200

    - Failure:

        ```json
        {
            "Message": "Not Found"
        }
        ```
        Status code: 404

        Description: The provided UUID is not known to this service.

- POST `/user/<uuid>/friends/remove`

    Remove a user's friend.

    Sample request body:

    ```json
    {
        "friend": "6392bacb8b00436682b33aaa134e83ac1"
    }
    ```

    Sample response:

    - Success:

        ```
        OK
        ```
        Status code: 200

    - Failure:

        - No such Friend

            ```json
            {
                "Message": "No such Friend"
            }
            ```
            Status code: 404

            Description: A friend with the UUID provided in the body does not exist for this user.

        - Not Found:

            ```json
            {
                "Message": "Not Found"
            }
            ```
            Status code: 404

            Description: The provided UUID is not known to this service.



