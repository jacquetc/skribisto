# Auto converter .qrc -> .py for developpers
import os


class QrcConverter:
    def __init__(self):
        _df = os.environ.get('PLUME_DEVELOP_FROM', None)
        if _df and os.path.exists(_df):
            self.build_resources(_df)

    def find_resources(self, srcdir):
        base = os.path.join(srcdir, 'src', 'plume')
        resources = []
        for root, _, files in os.walk(base):
            for name in files:
                if name.endswith('.qrc'):
                    resources.append(os.path.abspath(os.path.join(root, name)))

        return resources


    def resource_to_compiled_resource(self, resource):
        return resource.rpartition('.')[0] + '_rc.py'


    def build_resources(self, srcdir, info=None, summary=False):
        from subprocess import call

        def sub(match):
            ans = 'I(%s%s%s)' % (match.group(1), match.group(2), match.group(1))
            return ans

        num = 0

        resources = self.find_resources(srcdir)
        for resource in resources:
            compiled_resource = self.resource_to_compiled_resource(resource)
            if not os.path.exists(compiled_resource) or os.path.getmtime(resource) > os.path.getmtime(compiled_resource):
                if not summary:
                    print('\tCompiling resource', resource)
    #            buf = StringIO()
    #            compileUi(resource, buf)
    #            dat = buf.getvalue()
    #
    #            open(compiled_resource, 'w').write(dat)

                call(["pyrcc5", "-o", compiled_resource,  resource])

                num += 1
        if num:
            print('Compiled %d resources' % num)



