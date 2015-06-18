'''
Created on 16 avr. 2015

@author: Cyril Jacquet
'''


class DataError(OSError):

    '''root for DataErrors, only used to except any Data error, never raised'''
    pass


class DataUnableLoadFileError(DataError):

    '''An error if the file can't be loaded'''
    pass


class DataSaveFileAlreadyExistsError(DataError):

    '''An error if the file already exists'''
    pass


class DataSaveFilePathAccessError(DataError):

    '''An error if the file path can't be accessed anymore'''
    pass


class DataSQLiteError(DataError):

    '''An error if the SLite3 database raises an error'''
    pass
