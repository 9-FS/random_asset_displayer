# Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
import flask
from kfsconfig import kfsconfig
from kfslog import kfslog
import random
import typing


@kfslog.timeit()
def main(DEBUG: bool) -> None:
    env: dict[str, typing.Any]  # environment variables
    ENV_DEFAULT: dict[str, typing.Any]=\
    {
        "HOST": "::",
        "PORT": 80,
    }
    redirect_list: list[str]    # list of URL to redirect to or status code to return with empty response
    REDIRECT_LIST_DEFAULT: dict[str, str]=\
    {
        "content": ""
    }
    webapp: flask.Flask


    try:
        env          =kfsconfig.load_config(           config_filepaths=["./.env"],                     config_default=ENV_DEFAULT)                                     # load environment variables
        redirect_list=kfsconfig.load_config(env=False, config_filepaths=["./config/redirect_list.txt"], config_default=REDIRECT_LIST_DEFAULT)["content"].split("\n")    # load redirect_URL_list
    except ValueError:
        return
    webapp=flask.Flask(__name__)                                                                                                                                        # create webapp


    @webapp.route("/")                                  # add redirect to webapp root
    def redirect():
        redirect_target: str

        redirect_target=random.choice(redirect_list)    # URL to redirect to or status code to return with empty response
        if redirect_target.isdigit():                   # if status code:
            flask.abort(int(redirect_target))           # return status code
        else:                                           # if URL:
            return flask.redirect(redirect_target)      # redirect to random URL

    @webapp.route("/favicon.ico")   # browser tab icon
    def favicon():
        return flask.send_from_directory("../config/", "favicon.ico")

    @webapp.route("/ping")  # just send successful response, for testing
    def ping():
        return "Ping has been sucessful.", 200


    webapp.run(host=env["HOST"], port=env["PORT"])  # start webapp on port

    return