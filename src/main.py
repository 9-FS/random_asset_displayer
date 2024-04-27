# Copyright (c) 2023 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
import flask
from KFSconfig import KFSconfig
from KFSlog import KFSlog
import random


@KFSlog.timeit
def main(DEBUG: bool) -> None:
    port: int=6969
    redirect_list: list[str]    # list of URL to redirect to or status code to return with empty response
    webapp: flask.Flask


    try:
        redirect_list=KFSconfig.load_config("./config/redirect_list.txt", "").split("\n")   # load redirect_URL_list
    except FileNotFoundError as e:
        return
    webapp=flask.Flask(__name__)                                                            # create webapp


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


    webapp.run(host="::", port=port)    # start webapp on port
    
    return