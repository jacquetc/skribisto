# -*- coding: utf-8 -*-


import os



def find_forms(srcdir):
    base = os.path.join(srcdir, 'src', 'plume', "gui")
    forms = []
    for root, _, files in os.walk(base):
        for name in files:
            if name.endswith('.ui'):
                forms.append(os.path.abspath(os.path.join(root, name)))

    return forms

def form_to_compiled_form(form):
    return form.rpartition('.')[0]+'_ui.py'



def build_forms(srcdir, info=None, summary=False):
    import re
    from io import StringIO
    from PyQt5.uic import compileUi

    pat = re.compile(r'''(['"]):/images/([^'"]+)\1''')
    def sub(match):
        ans = 'I(%s%s%s)'%(match.group(1), match.group(2), match.group(1))
        return ans

    num = 0
    transdef_pat = re.compile(r'^\s+_translate\s+=\s+QtCore.QCoreApplication.translate$', flags=re.M)
    transpat = re.compile(r'_translate\s*\(.+?,\s+"(.+?)(?<!\\)"\)', re.DOTALL)

    # Ensure that people running from source have all their forms rebuilt for
    # the qt5 migration
    #force_compile = check_for_migration and not gprefs.get('migrated_forms_to_qt5', False)

    forms = find_forms(srcdir)
    for form in forms:
        compiled_form = form_to_compiled_form(form)
        if not os.path.exists(compiled_form) or os.stat(form).st_mtime > os.stat(compiled_form).st_mtime:
            if not summary:
                print('\tCompiling form', form)
            buf = StringIO()
            compileUi(form, buf)
            dat = buf.getvalue()
            dat = dat.replace('import images_rc', '')
            dat = transdef_pat.sub('', dat)
            dat = transpat.sub(r'_("\1")', dat)
            dat = dat.replace('_("MMM yyyy")', '"MMM yyyy"')
            dat = dat.replace('_("d MMM yyyy")', '"d MMM yyyy"')
            dat = pat.sub(sub, dat)

            open(compiled_form, 'w').write(dat)
            num += 1
    if num:
        print('Compiled %d forms' % num)

        

_df = os.environ.get('PLUME_DEVELOP_FROM', None)
if _df and os.path.exists(_df):
    build_forms(_df)