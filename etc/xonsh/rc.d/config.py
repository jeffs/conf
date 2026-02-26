from xonsh.built_ins import XSH

env = XSH.env

if env.get('XONSH_LOGIN'):
    env['XXX_FOO'] = 'login'

if env.get('XONSH_INTERACTIVE'):
    XSH.aliases['c'] = 'cd'

