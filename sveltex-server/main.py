import json
from aiohttp import web 
import aiohttp_cors
import asyncio
from setup.setup import setup 
async def check_admin_set():
    #Checking if the adminstrator account has been set
    default=open("config/config.json")
    default_data=json.load(default)
    
    if(not default_data["configured"]):
        #Config has not yet been configured, so we must go through setup.
        await setup()

async def main():
    await check_admin_set()

async def launch_server():
    app = web.Application()
    app.add_routes([
       # web.post("/get-session-key",get_session_key_wrapper),

    ])
    cors = aiohttp_cors.setup(app, defaults={
    "*": aiohttp_cors.ResourceOptions(
            allow_credentials=True,
            expose_headers="*",
            allow_headers="*"
        )
    })


if __name__=="__main__":
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    loop.run_until_complete(main())    
else:
    print(__name__)