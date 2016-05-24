'''
Created on 13 february 2016

@author:  Cyril Jacquet
'''

class DbError:
    #
    # Common Db error codes and status handling
    #
    R_ERROR = 0                # Error return - check get_status(). MUST BE ZERO e.g. get_sheet_order
    R_OK = 1                   # Success return (some functions return another non zero value e.g. a sheet_id)

    # Error codes
    E_INVTYPE = 1              # Invalid parameter type.
    E_INVPARAM = 2             # Invalid parameter value.
    E_INTERNERR = 3            # Internal logic error or unexpected error
    E_RECNOTFOU = 4            # Requested record not found

    def __init__(self):
        # Class variables  All vars are considered private - use the setter / getter to access.
        self.i_status = 0               # Error status
        self.i_param = 0                # Error parameter number (self = 0)
        self.s_err_msg = ''

    def get_status(self):
        #
        # Returns a tuple with the last error status, message, and parameter number (if any).
        # If is only meaningful to call this if a function returns R_ERROR
        #
        if self.i_status == DbErr.E_INVTYPE:
            s_msg = 'Invalid parameter type (parameter {0})'.format(self.i_param)
        elif self.i_status == DbErr.E_INVPARAM:
            s_msg = 'Invalid parameter value (parameter {0})'.format(self.i_param)
        elif self.i_status == DbErr.E_INTERNERR:
            s_msg = 'Internal logic error'
        else:
            s_msg = 'Unhandled error'

        if self.s_err_msg != '':
            s_msg += '. ' + self.s_err_msg

        return (self.i_status, self.i_param, self.s_err_msg)

    def status(self) -> str:
        #
        # Returns get_status, nicely formatted
        #
        t_err = self.get_status()
        s_msg = 'Error: {0}. '.format(t_err[0]) + t_err[2]

        return s_msg

    def set_status(self, i_status: int, i_param: int, s_err_msg: str):
        #
        # Set the error status.
        #
        self.i_status = i_status
        self.i_param = i_param
        self.s_err_msg = s_err_msg
