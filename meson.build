project('MAPS', 'cpp',
  version : '0.1',
  default_options : ['warning_level=3'])

opencv_dep = dependency('opencv4')

exe = executable('MAPS', 'maps.cpp',
  install : true, dependencies : opencv_dep)

test('basic', exe)
