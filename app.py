import click
from flask import Flask, request, jsonify
from flask.cli import AppGroup
from flask_marshmallow import Marshmallow
from flask_restful import Api
from flask_sqlalchemy import SQLAlchemy
from mojang import API

# ----- Variables

VERSION = "1.0.0"

SESSION_SERVER_URL = "https://sessionserver.mojang.com/session/minecraft/profile/"

# ----- No variables below this line

# Flask initialization
app = Flask(__name__)
api = Api(app)
app.config['SQLALCHEMY_DATABASE_URI'] = 'sqlite:///players.db'
app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = False
db = SQLAlchemy(app)
ma = Marshmallow(app)
user_cli = AppGroup('user')

# Mojang API Instance
mojang_api = API()

version_major = VERSION.split(".")[0]


class User(db.Model):
    __tablename__ = "user"
    uuid = db.Column(db.String(32), unique=True, primary_key=True)
    online = db.Column(db.Boolean())
    friends = db.relationship('Friend')

    def addFriend(self, friend):
        self.friends.append(friend)

    def __init__(self, uuid):
        self.uuid = uuid

    def serialize(self):
        friends = []
        for f in self.friends:
            friends.append(f.serialize())
        return jsonify({
            'friends': friends,
            'online': self.online,
            'uuid': self.uuid
        })


class Friend(db.Model):
    user = db.Column(db.String(32), db.ForeignKey('user.uuid'))
    uuid = db.Column(db.String(32), primary_key=True)

    def __init__(self, uuid):
        self.uuid = uuid

    def serialize(self):
        return {
            'uuid': self.uuid
        }


class UserSchema(ma.SQLAlchemySchema):
    class Meta:
        fields = ('id', 'uuid', 'online', 'friends')


user_schema = UserSchema()
users_schema = UserSchema(many=True)


@app.route("/")
def welcome():
    return jsonify({
        'about': "Welcome fellow traveler!",
        'documentation': "https://github.com/AxolotlClient/AxolotlClient-mod/wiki/api/",
        'name': "axolotlclient-api",
        'version': VERSION
    })


@app.route("/v" + version_major + "/<uuid>", methods=['GET'])
def get_user(uuid):
    uuid.replace("-", "")

    user = db.session.execute(db.select(User).filter_by(uuid=uuid)).one_or_none()[0]

    if user:
        return user.serialize()

    response = jsonify({"Message": "Not Found"})
    response.status_code = 404
    return response


@app.route("/v" + version_major + "/<uuid>", methods=['POST'])
def update_user(uuid):
    uuid.replace("-", "")

    user = db.session.execute(db.select(User).filter_by(uuid=uuid)).one_or_none()[0]

    if not user:

        name = mojang_api.get_username(uuid)
        if not name:
            response = jsonify({"error_message": "UUID "+uuid+" does not appear to be valid!"})
            response.status_code = 401
            return response

        print("Creating new user for " + uuid)
        user = User(uuid)
        online = request.json['online']
        user.online = online
        db.session.add(user)
        db.session.commit()
        response = app.make_response("OK")
        response.status_code = 201
        return response
    else:
        print("updating existing user " + uuid)
        online = request.json['online']
        user.online = online
        db.session.commit()
        response = app.make_response("OK")
        response.status_code = 200
        return response


@app.route("/v" + version_major + "/count")
def get_users_counts():
    return jsonify({
        "total": len(User.query.all()),
        "online": len(User.query.filter_by(online=True).all())
    })

#
# @app.route("/v" + version_major + "/all")
# def all_users():
#     return jsonify(User.query.all())


@app.route("/v" + version_major + "/<uuid>/friends", methods=['GET'])
def get_friends(uuid):
    uuid.replace("-", "")
    try:
        user = db.one_or_404(db.select(User).filter_by(uuid=uuid))
        friends = []
        for f in user.friends:
            friends.append(f.serialize())
        return jsonify(friends)
    except Exception as _:
        print(_)
        print("UUID: " + uuid)
        response = jsonify({"Message": "Not Found"})
        response.status_code = 404
        return response


@app.route("/v" + version_major + "/<uuid>/friends", methods=['POST'])
def add_friend(uuid):
    uuid.replace("-", "")
    try:
        user = db.one_or_404(db.select(User).filter_by(uuid=uuid))
        friend = Friend(request.json["friend"].replace("-", ""))
        user.friends.append(friend)
        db.session.commit()
        response = app.make_response("OK")
        response.status_code = 200
        return response
    except Exception as _:
        print(_)
        print("UUID: " + uuid)
        response = jsonify({"Message": "Not Found"})
        response.status_code = 404
        return response


@app.route("/v" + version_major + "/<uuid>/friends/remove", methods=['POST'])
def remove_friend(uuid):
    uuid.replace("-", "")
    try:
        user = db.one_or_404(db.select(User).filter_by(uuid=uuid))
        friend_uuid = request.json["friend"]
        for friend in user.friends:
            if friend.uuid == friend_uuid:
                user.friends.remove(friend)
                db.session.commit()
                response = app.make_response("OK")
                response.status_code = 200
                return response
        response = jsonify({"Message": "No such Friend"})
        response.status_code = 404
        return response
    except Exception as _:
        print(_)
        print("UUID: " + uuid)
        response = jsonify({"Message": "Not Found"})
        response.status_code = 404
        return response


@user_cli.command('initdb')
def initdb_command():
    """Initializes the database."""
    with app.test_request_context():
        db.create_all()
    print('Initialized the database.')


@user_cli.command('remove')
@click.argument("uuid")
def remove_user_command(uuid):
    if "-" in uuid:
        uuid.replace("-", "")
    with app.test_request_context():
        user = db.session.execute(db.select(User).filter_by(uuid=uuid)).one_or_none()[0]
        if user:
            print("Deleting" + user.uuid)
            db.session.delete(user)
            db.session.commit()
        else:
            print("User " + uuid + " not found!")


@app.cli.command("list")
def list_command():
    with app.test_request_context():
        print(jsonify(User.query.all()).json)


app.cli.add_command(user_cli)


if __name__ == '__main__':
    app.run(debug=True)
