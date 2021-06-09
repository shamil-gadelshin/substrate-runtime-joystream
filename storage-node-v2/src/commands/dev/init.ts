import { flags } from '@oclif/command'
import { hireStorageWorkingGroupLead } from '../../services/runtime/hireLead'
import ApiCommandBase from '../../command-base/ApiCommandBase'

//TODO: consider renaming the command - DevHireLeader
export default class DevInit extends ApiCommandBase {
  static description =
    'Initialize development environment. Sets Alice as storage working group leader.'

  static flags = {
    help: flags.help({ char: 'h' }),
  }

  async run(): Promise<void> {
    await this.ensureDevelopmentChain()

    await hireStorageWorkingGroupLead()
  }
}